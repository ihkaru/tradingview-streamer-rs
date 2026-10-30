//! Protocol submodule re-exports and high-level wire framing utilities.

pub mod framing;
pub mod messages;
pub mod parser;

pub use framing::{format_frame, format_heartbeat, parse_frames, InboundFrame};
pub use messages::{
    build_symbol_spec, chart_create_session, create_series, encode_rpc, quote_add_symbols,
    quote_create_session, quote_fast_symbols, quote_hibernate_all, quote_set_fields,
    resolve_symbol, set_auth_token, set_locale, switch_timezone, STANDARD_QUOTE_FIELDS,
};
pub use parser::parse_message_payload;
