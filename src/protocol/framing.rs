//! TradingView WebSocket protocol framing (`~m~<length>~m~<content>`).

use crate::error::{Error, Result};

/// A decoded packet from the TradingView WebSocket stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InboundFrame {
    /// Heartbeat ping from server with sequence counter: `~h~<num>`.
    Heartbeat(i64),
    /// Standard JSON message payload.
    Message(String),
}

/// Prepends the TradingView wire framing header `~m~<length>~m~` to a payload.
#[inline]
pub fn format_frame(payload: &str) -> String {
    format!("~m~{}~m~{}", payload.len(), payload)
}

/// Formats a heartbeat response message for a given heartbeat sequence number.
#[inline]
pub fn format_heartbeat(sequence: i64) -> String {
    let content = format!("~h~{sequence}");
    format!("~m~{}~m~{}", content.len(), content)
}

/// Parses a raw WebSocket string containing one or more framed packets.
///
/// TradingView frames packets using `~m~<length>~m~<payload>`.
/// Multiple framed messages can arrive concatenated within a single WebSocket text frame.
pub fn parse_frames(raw: &str) -> Result<Vec<InboundFrame>> {
    let mut frames = Vec::new();
    let mut remainder = raw;

    while !remainder.is_empty() {
        if !remainder.starts_with("~m~") {
            // If remainder does not start with ~m~, scan forward to next marker or fail
            if let Some(pos) = remainder.find("~m~") {
                remainder = &remainder[pos..];
            } else {
                break;
            }
        }

        // Find the second "~m~"
        let after_prefix = &remainder[3..];
        let Some(second_delim_idx) = after_prefix.find("~m~") else {
            return Err(Error::Protocol(format!(
                "Malformed frame header: missing closing ~m~ delimiter in '{remainder}'"
            )));
        };

        let length_str = &after_prefix[..second_delim_idx];
        let payload_len: usize = length_str.parse().map_err(|e| {
            Error::Protocol(format!("Invalid frame length header '{length_str}': {e}"))
        })?;

        let payload_start = 3 + second_delim_idx + 3;
        if remainder.len() < payload_start + payload_len {
            return Err(Error::Protocol(format!(
                "Truncated frame: expected {payload_len} bytes but only {} available",
                remainder.len().saturating_sub(payload_start)
            )));
        }

        let payload = &remainder[payload_start..payload_start + payload_len];

        // Check if payload is a heartbeat ping: "~h~<num>"
        if let Some(hb_str) = payload.strip_prefix("~h~") {
            if let Ok(seq) = hb_str.parse::<i64>() {
                frames.push(InboundFrame::Heartbeat(seq));
            } else {
                frames.push(InboundFrame::Message(payload.to_string()));
            }
        } else {
            frames.push(InboundFrame::Message(payload.to_string()));
        }

        remainder = &remainder[payload_start + payload_len..];
    }

    Ok(frames)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_frame() {
        let payload = r#"{"m":"test","p":[]}"#;
        let framed = format_frame(payload);
        assert_eq!(framed, format!("~m~{}~m~{payload}", payload.len()));
    }

    #[test]
    fn test_format_heartbeat() {
        let hb = format_heartbeat(42);
        assert_eq!(hb, "~m~5~m~~h~42");
    }

    #[test]
    fn test_parse_single_frame() {
        let payload = r#"{"m":"ping","p":[]}"#;
        let raw = format_frame(payload);
        let parsed = parse_frames(&raw).expect("Parsing should succeed");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0], InboundFrame::Message(payload.to_string()));
    }

    #[test]
    fn test_parse_heartbeat_frame() {
        let raw = "~m~5~m~~h~99";
        let parsed = parse_frames(raw).expect("Heartbeat parse should succeed");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0], InboundFrame::Heartbeat(99));
    }

    #[test]
    fn test_parse_multiple_concatenated_frames() {
        let payload = r#"{"m":"test","p":[]}"#;
        let msg_frame = format_frame(payload);
        let hb_frame = format_heartbeat(10);
        let raw = format!("{hb_frame}{msg_frame}");

        let parsed = parse_frames(&raw).expect("Multi-frame parse should succeed");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], InboundFrame::Heartbeat(10));
        assert_eq!(parsed[1], InboundFrame::Message(payload.to_string()));
    }
}
