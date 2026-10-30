//! Inbound payload parser for TradingView WebSocket stream responses.

use serde_json::Value;
use crate::error::{Error, Result};
use crate::types::{OhlcvCandle, QuoteUpdate, StreamEvent};

/// Parses a JSON text message from TradingView into zero or more high-level [`StreamEvent`]s.
pub fn parse_message_payload(json_str: &str, series_id: &str) -> Result<Vec<StreamEvent>> {
    let root: Value = serde_json::from_str(json_str)?;
    let mut events = Vec::new();

    let Some(method) = root.get("m").and_then(Value::as_str) else {
        return Ok(events);
    };

    let params = root.get("p").and_then(Value::as_array);

    match method {
        "timescale_update" => {
            if let Some(params) = params {
                if let Some(candles) = extract_timescale_candles(params, series_id) {
                    if !candles.is_empty() {
                        events.push(StreamEvent::Candles(candles));
                    }
                }
            }
        }
        "du" => {
            if let Some(params) = params {
                if let Some(candles) = extract_timescale_candles(params, series_id) {
                    if !candles.is_empty() {
                        events.push(StreamEvent::Candles(candles));
                    }
                }
                extract_study_events(params, &mut events);
            }
        }
        "qsd" => {
            if let Some(params) = params {
                if let Some(quote) = extract_quote_update(params) {
                    events.push(StreamEvent::Quote(quote));
                }
            }
        }
        "symbol_resolved" => {
            if let Some(params) = params {
                if let Some(evt) = extract_symbol_resolved(params) {
                    events.push(evt);
                }
            }
        }
        "series_completed" => {
            events.push(StreamEvent::SeriesCompleted);
        }
        "critical_error" => {
            let err_msg = params
                .and_then(|p| p.first())
                .and_then(Value::as_str)
                .unwrap_or("Unknown critical error from TradingView server");
            return Err(Error::Protocol(err_msg.to_string()));
        }
        _ => {}
    }

    Ok(events)
}

/// Extracts OHLCV candles from `timescale_update` or `du` parameter list.
fn extract_timescale_candles(params: &[Value], series_id: &str) -> Option<Vec<OhlcvCandle>> {
    let data_map = params.get(1)?.as_object()?;
    let series_obj = data_map.get(series_id)?.as_object()?;
    let s_array = series_obj.get("s")?.as_array()?;

    let mut candles = Vec::with_capacity(s_array.len());
    for item in s_array {
        if let Some(candle) = parse_candle_item(item) {
            candles.push(candle);
        }
    }

    Some(candles)
}

/// Parses an individual candle node `{"i": index, "v": [ts, o, h, l, c, v?]}`.
fn parse_candle_item(item: &Value) -> Option<OhlcvCandle> {
    let index = item.get("i")?.as_i64()?;
    let v_array = item.get("v")?.as_array()?;

    if v_array.len() < 5 {
        return None;
    }

    let timestamp = v_array[0].as_f64()? as i64;
    let open = v_array[1].as_f64()?;
    let high = v_array[2].as_f64()?;
    let low = v_array[3].as_f64()?;
    let close = v_array[4].as_f64()?;
    let volume = if v_array.len() > 5 {
        v_array[5].as_f64()
    } else {
        None
    };

    Some(OhlcvCandle {
        index,
        timestamp,
        open,
        high,
        low,
        close,
        volume,
    })
}

/// Extracts indicator study updates from `du` parameter payload.
fn extract_study_events(params: &[Value], events: &mut Vec<StreamEvent>) {
    let Some(data_map) = params.get(1).and_then(Value::as_object) else {
        return;
    };

    for (key, val) in data_map {
        if key.starts_with("st") {
            events.push(StreamEvent::Study {
                study_id: key.clone(),
                data: val.clone(),
            });
        }
    }
}

/// Extracts real-time quote metrics from `qsd` payload.
fn extract_quote_update(params: &[Value]) -> Option<QuoteUpdate> {
    let payload = params.get(1)?.as_object()?;
    let v_map = payload.get("v")?.as_object()?;
    let symbol = payload.get("n").and_then(Value::as_str).map(clean_symbol_name);

    Some(QuoteUpdate {
        symbol,
        last_price: v_map.get("lp").and_then(Value::as_f64),
        change: v_map.get("ch").and_then(Value::as_f64),
        change_percentage: v_map.get("chp").and_then(Value::as_f64),
        realtime_change: v_map.get("rtc").and_then(Value::as_f64),
        realtime_change_percentage: v_map.get("rchp").and_then(Value::as_f64),
        bid: v_map.get("bid").and_then(Value::as_f64),
        ask: v_map.get("ask").and_then(Value::as_f64),
        spread: v_map.get("spread").and_then(Value::as_f64),
        volume: v_map.get("volume").and_then(Value::as_f64),
        high_price: v_map.get("high_price").and_then(Value::as_f64),
        low_price: v_map.get("low_price").and_then(Value::as_f64),
        open_price: v_map.get("open_price").and_then(Value::as_f64),
        prev_close_price: v_map.get("prev_close_price").and_then(Value::as_f64),
        currency: v_map.get("currency_code").and_then(Value::as_str).map(ToString::to_string),
        exchange: v_map.get("exchange").and_then(Value::as_str).map(ToString::to_string),
        last_price_time: v_map.get("lp_time").and_then(Value::as_i64),
        raw_fields: Value::Object(v_map.clone()),
    })
}

/// Normalizes symbol names, unpacking serialized JSON specs (`={"symbol":...}`) into plain symbols.
fn clean_symbol_name(raw: &str) -> String {
    if let Some(spec) = raw.strip_prefix('=') {
        if let Ok(val) = serde_json::from_str::<Value>(spec) {
            if let Some(sym) = val.get("symbol").and_then(Value::as_str) {
                return sym.to_string();
            }
        }
    }
    raw.to_string()
}

/// Extracts `symbol_resolved` metadata.
fn extract_symbol_resolved(params: &[Value]) -> Option<StreamEvent> {
    let symbol = params.get(1)?.as_str()?.to_string();
    let metadata = params.get(2).cloned().unwrap_or(Value::Null);

    Some(StreamEvent::SymbolResolved { symbol, metadata })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timescale_update() {
        let json_data = r#"{
            "m": "timescale_update",
            "p": [
                "cs_test",
                {
                    "sds_1": {
                        "s": [
                            {"i": 100, "v": [1700000000.0, 150.0, 155.0, 149.0, 154.5, 12000.0]},
                            {"i": 101, "v": [1700000060.0, 154.5, 156.0, 153.5, 155.2, 8500.0]}
                        ]
                    }
                }
            ]
        }"#;

        let events = parse_message_payload(json_data, "sds_1").expect("Parse should succeed");
        assert_eq!(events.len(), 1);
        if let StreamEvent::Candles(candles) = &events[0] {
            assert_eq!(candles.len(), 2);
            assert_eq!(candles[0].index, 100);
            assert_eq!(candles[0].timestamp, 1700000000);
            assert_eq!(candles[0].open, 150.0);
            assert_eq!(candles[0].high, 155.0);
            assert_eq!(candles[0].low, 149.0);
            assert_eq!(candles[0].close, 154.5);
            assert_eq!(candles[0].volume, Some(12000.0));
        } else {
            panic!("Expected Candles event");
        }
    }

    #[test]
    fn test_parse_candle_without_volume() {
        let json_data = r#"{
            "m": "timescale_update",
            "p": [
                "cs_test",
                {
                    "sds_1": {
                        "s": [
                            {"i": 1, "v": [1700000000.0, 10.0, 12.0, 9.0, 11.0]}
                        ]
                    }
                }
            ]
        }"#;

        let events = parse_message_payload(json_data, "sds_1").expect("Parse should succeed");
        if let StreamEvent::Candles(candles) = &events[0] {
            assert_eq!(candles.len(), 1);
            assert_eq!(candles[0].volume, None);
        } else {
            panic!("Expected Candles event");
        }
    }
}
