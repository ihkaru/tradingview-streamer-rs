use std::collections::HashSet;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

use super::multiplexer::SymbolRouter;
use super::types::PoolConfig;
use crate::client::{ClientConfig, TvWebSocketClient};
use crate::error::{Error, Result};
use crate::protocol::framing::{format_heartbeat, parse_frames, InboundFrame};
use crate::protocol::messages::{
    build_symbol_spec, quote_add_symbols, quote_create_session, quote_hibernate_all,
    quote_remove_symbols, quote_set_fields, set_auth_token, set_locale,
    STANDARD_QUOTE_FIELDS,
};
use crate::protocol::parser::parse_message_payload;
use crate::session::generate_session_id;
use crate::types::{PriceAdjustment, StreamEvent};

pub enum WorkerCommand {
    Subscribe(String),
    Unsubscribe(String),
    Shutdown,
}

/// Dedicated connection worker managing a single WebSocket session for up to N symbols.
#[derive(Debug)]
pub struct PoolWorker {
    pub worker_id: usize,
    cmd_tx: mpsc::Sender<WorkerCommand>,
}

impl PoolWorker {
    pub async fn spawn(
        worker_id: usize,
        config: PoolConfig,
        router: SymbolRouter,
    ) -> Result<Self> {
        let (cmd_tx, cmd_rx) = mpsc::channel(128);
        let mut client_cfg = ClientConfig::default();
        client_cfg.ws_url = config.ws_url.clone();
        let client = TvWebSocketClient::new(client_cfg);
        let ws_stream = client.connect().await?;
        let quote_session = generate_session_id("qs");

        tokio::spawn(async move {
            if let Err(e) = run_worker_loop(
                ws_stream,
                cmd_rx,
                router,
                config,
                quote_session,
            )
            .await
            {
                tracing::debug!("Worker {} loop exited: {e}", worker_id);
            }
        });

        Ok(Self { worker_id, cmd_tx })
    }

    pub async fn subscribe(&self, symbol: &str) -> Result<()> {
        self.cmd_tx
            .send(WorkerCommand::Subscribe(symbol.to_string()))
            .await
            .map_err(|_| Error::Protocol("Worker channel closed".into()))
    }

    pub async fn unsubscribe(&self, symbol: &str) -> Result<()> {
        self.cmd_tx
            .send(WorkerCommand::Unsubscribe(symbol.to_string()))
            .await
            .map_err(|_| Error::Protocol("Worker channel closed".into()))
    }
}

async fn run_worker_loop<S>(
    mut ws: S,
    mut cmd_rx: mpsc::Receiver<WorkerCommand>,
    router: SymbolRouter,
    config: PoolConfig,
    qs: String,
) -> Result<()>
where
    S: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error>
        + StreamExt<Item = std::result::Result<Message, tokio_tungstenite::tungstenite::Error>>
        + Unpin,
{
    init_worker_session(&mut ws, &qs).await?;
    let mut subscribed = HashSet::new();

    loop {
        tokio::select! {
            Some(cmd) = cmd_rx.recv() => {
                match cmd {
                    WorkerCommand::Subscribe(sym) => {
                        let spec = build_symbol_spec(&sym, config.session_type, PriceAdjustment::Splits);
                        let msg = quote_add_symbols(&qs, &spec);
                        ws.send(Message::Text(msg.into())).await.map_err(Error::WebSocket)?;
                        subscribed.insert(sym);
                    }
                    WorkerCommand::Unsubscribe(sym) => {
                        let msg = quote_remove_symbols(&qs, &sym);
                        let _ = ws.send(Message::Text(msg.into())).await;
                        subscribed.remove(&sym);
                    }
                    WorkerCommand::Shutdown => break,
                }
            }
            msg_res = ws.next() => {
                let msg = match msg_res {
                    Some(Ok(m)) => m,
                    Some(Err(e)) => return Err(Error::WebSocket(e)),
                    None => break,
                };

                if let Message::Text(text) = msg {
                    handle_worker_text(&mut ws, &text, &router).await?;
                }
            }
        }
    }

    Ok(())
}

async fn init_worker_session<S>(ws: &mut S, qs: &str) -> Result<()>
where
    S: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
{
    let msgs = [
        set_auth_token("unauthorized_user_token"),
        set_locale("en", "US"),
        quote_create_session(qs),
        quote_set_fields(qs, STANDARD_QUOTE_FIELDS),
        quote_hibernate_all(qs),
    ];
    for m in msgs {
        ws.send(Message::Text(m.into())).await.map_err(Error::WebSocket)?;
    }
    Ok(())
}

async fn handle_worker_text<S>(
    ws: &mut S,
    text: &str,
    router: &SymbolRouter,
) -> Result<()>
where
    S: SinkExt<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin,
{
    let frames = parse_frames(text)?;
    for frame in frames {
        match frame {
            InboundFrame::Heartbeat(seq) => {
                let pong = format_heartbeat(seq);
                ws.send(Message::Text(pong.into())).await.map_err(Error::WebSocket)?;
            }
            InboundFrame::Message(payload) => {
                let events = parse_message_payload(&payload, "sds_worker")?;
                for evt in events {
                    if let StreamEvent::Quote(quote) = evt {
                        if let Some(sym) = quote.symbol.clone() {
                            router.dispatch(&sym, quote).await;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
