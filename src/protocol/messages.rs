//! Outbound command message builders for TradingView WebSocket protocol.

use serde_json::json;
use crate::protocol::framing::format_frame;
use crate::types::{SessionType, Timeframe};

/// Standard quote fields requested from TradingView.
pub const STANDARD_QUOTE_FIELDS: &[&str] = &[
    "ch", "chp", "current_session", "description", "local_description",
    "language", "exchange", "fractional", "is_tradable", "lp",
    "lp_time", "minmov", "minmove2", "original_name", "pricescale",
    "pro_name", "short_name", "type", "update_mode", "volume",
    "currency_code", "rchp", "rtc", "bid", "ask", "spread",
    "open_price", "high_price", "low_price", "prev_close_price",
];

/// Encodes an outbound RPC function call into framed TradingView wire format.
#[inline]
pub fn encode_rpc(func: &str, params: serde_json::Value) -> String {
    let payload = json!({
        "m": func,
        "p": params
    }).to_string();
    format_frame(&payload)
}

/// Builds `set_auth_token` command message.
pub fn set_auth_token(jwt: &str) -> String {
    encode_rpc("set_auth_token", json!([jwt]))
}

/// Builds `set_locale` command message.
pub fn set_locale(lang: &str, country: &str) -> String {
    encode_rpc("set_locale", json!([lang, country]))
}

/// Builds `chart_create_session` command message.
pub fn chart_create_session(chart_session: &str) -> String {
    encode_rpc("chart_create_session", json!([chart_session, ""]))
}

/// Builds `quote_create_session` command message.
pub fn quote_create_session(quote_session: &str) -> String {
    encode_rpc("quote_create_session", json!([quote_session]))
}

/// Builds `quote_set_fields` command message.
pub fn quote_set_fields(quote_session: &str, fields: &[&str]) -> String {
    let mut params = vec![json!(quote_session)];
    for field in fields {
        params.push(json!(field));
    }
    encode_rpc("quote_set_fields", json!(params))
}

/// Builds `quote_hibernate_all` command message.
pub fn quote_hibernate_all(quote_session: &str) -> String {
    encode_rpc("quote_hibernate_all", json!([quote_session]))
}

/// Builds `quote_add_symbols` with symbol specification string.
pub fn quote_add_symbols(quote_session: &str, resolve_str: &str) -> String {
    encode_rpc("quote_add_symbols", json!([quote_session, resolve_str]))
}

/// Builds `quote_fast_symbols` command message.
pub fn quote_fast_symbols(quote_session: &str, symbol: &str) -> String {
    encode_rpc("quote_fast_symbols", json!([quote_session, symbol]))
}

/// Constructs the symbol specification JSON string used by `resolve_symbol` and `quote_add_symbols`.
///
/// ## Extended Hours Fix (Issue #63)
/// TradingView defaults to `"session": "regular"` when omitted in `mnwato/tradingview-scraper`.
/// Passing `"session": "extended"` instructs TradingView to stream Extended Trading Hours (ETH)
/// including pre-market and after-hours bars.
pub fn build_symbol_spec(symbol: &str, session: SessionType, adjustment: &str) -> String {
    let spec = json!({
        "adjustment": adjustment,
        "symbol": symbol,
        "session": session.as_str()
    });
    format!("={spec}")
}

/// Builds `resolve_symbol` command message with explicit session type support.
pub fn resolve_symbol(
    chart_session: &str,
    symbol_alias: &str,
    symbol: &str,
    session: SessionType,
    adjustment: &str,
) -> String {
    let spec = build_symbol_spec(symbol, session, adjustment);
    encode_rpc("resolve_symbol", json!([chart_session, symbol_alias, spec]))
}

/// Builds `create_series` command message to start bar calculations.
pub fn create_series(
    chart_session: &str,
    series_id: &str,
    series_type: &str,
    symbol_alias: &str,
    timeframe: &Timeframe,
    bar_count: u32,
) -> String {
    encode_rpc(
        "create_series",
        json!([
            chart_session,
            series_id,
            series_type,
            symbol_alias,
            timeframe.as_wire_str(),
            bar_count,
            ""
        ]),
    )
}

/// Builds `switch_timezone` command message.
pub fn switch_timezone(chart_session: &str, timezone: &str) -> String {
    encode_rpc("switch_timezone", json!([chart_session, timezone]))
}

/// Builds `create_study` command message to attach an indicator study to a chart session.
pub fn create_study(
    chart_session: &str,
    study_id: &str,
    study_alias: &str,
    series_id: &str,
    indicator: &str,
    inputs: serde_json::Value,
) -> String {
    encode_rpc(
        "create_study",
        json!([chart_session, study_id, study_alias, series_id, indicator, inputs]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_symbol_spec_regular_session() {
        let spec = build_symbol_spec("NASDAQ:AAPL", SessionType::Regular, "splits");
        assert!(spec.starts_with('='));
        let parsed: serde_json::Value = serde_json::from_str(&spec[1..]).unwrap();
        assert_eq!(parsed["symbol"], "NASDAQ:AAPL");
        assert_eq!(parsed["adjustment"], "splits");
        assert_eq!(parsed["session"], "regular");
    }

    #[test]
    fn test_build_symbol_spec_extended_session_issue_63() {
        let spec = build_symbol_spec("NASDAQ:AAPL", SessionType::Extended, "splits");
        assert!(spec.starts_with('='));
        let parsed: serde_json::Value = serde_json::from_str(&spec[1..]).unwrap();
        assert_eq!(parsed["symbol"], "NASDAQ:AAPL");
        assert_eq!(parsed["adjustment"], "splits");
        // Issue #63 resolution invariant:
        assert_eq!(parsed["session"], "extended");
    }

    #[test]
    fn test_resolve_symbol_command_encoding() {
        let cmd = resolve_symbol(
            "cs_test123",
            "sds_sym_1",
            "NASDAQ:AAPL",
            SessionType::Extended,
            "splits",
        );
        assert!(cmd.starts_with("~m~"));
        assert!(cmd.contains("resolve_symbol"));
        // In the outer JSON array, the inner JSON string is escaped
        assert!(cmd.contains(r#"\"session\":\"extended\""#));
    }

    #[test]
    fn test_create_series_command_encoding() {
        let cmd = create_series(
            "cs_test123",
            "sds_1",
            "s1",
            "sds_sym_1",
            &Timeframe::H1,
            500,
        );
        assert!(cmd.starts_with("~m~"));
        assert!(cmd.contains("create_series"));
        assert!(cmd.contains(r#""60""#)); // 1 hour -> "60"
        assert!(cmd.contains("500"));
    }

    #[test]
    fn test_create_study_command_encoding() {
        let cmd = create_study(
            "cs_test123",
            "st1",
            "st1",
            "sds_1",
            "Volume@tv-basicstudies-246",
            json!({"length": 20}),
        );
        assert!(cmd.starts_with("~m~"));
        assert!(cmd.contains("create_study"));
        assert!(cmd.contains("Volume@tv-basicstudies-246"));
        assert!(cmd.contains(r#"\"length\":20"#) || cmd.contains(r#"{"length":20}"#));
    }
}
