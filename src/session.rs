//! Session identifier generator and session state container.

use rand::Rng;

/// Character set for random session token generation (lowercase ASCII).
const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const RANDOM_LENGTH: usize = 12;

/// Generates a random session identifier composed of a prefix followed by 12 random lowercase letters.
///
/// Example: `generate_session_id("cs_")` -> `"cs_abcedfghijkl"`
pub fn generate_session_id(prefix: &str) -> String {
    let mut rng = rand::thread_rng();
    let random_suffix: String = (0..RANDOM_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    format!("{prefix}{random_suffix}")
}

/// Container for active TradingView chart and quote session tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionTokens {
    /// Chart session token (e.g. `cs_xxxxxxxxxxxx`)
    pub chart_session: String,
    /// Quote session token (e.g. `qs_xxxxxxxxxxxx`)
    pub quote_session: String,
    /// Main series watcher ID (default: `"sds_1"`)
    pub series_id: String,
    /// Symbol watcher alias (default: `"sds_sym_1"`)
    pub symbol_alias: String,
}

impl Default for SessionTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionTokens {
    /// Generates a new unique pair of chart and quote session tokens.
    pub fn new() -> Self {
        Self {
            chart_session: generate_session_id("cs_"),
            quote_session: generate_session_id("qs_"),
            series_id: "sds_1".to_string(),
            symbol_alias: "sds_sym_1".to_string(),
        }
    }

    /// Creates session tokens with explicitly provided session IDs (useful for hermetic testing).
    pub fn with_explicit_tokens(chart_session: impl Into<String>, quote_session: impl Into<String>) -> Self {
        Self {
            chart_session: chart_session.into(),
            quote_session: quote_session.into(),
            series_id: "sds_1".to_string(),
            symbol_alias: "sds_sym_1".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_session_id_format() {
        let cs = generate_session_id("cs_");
        assert!(cs.starts_with("cs_"));
        assert_eq!(cs.len(), 3 + 12);
        assert!(cs[3..].chars().all(|c| c.is_ascii_lowercase()));

        let qs = generate_session_id("qs_");
        assert!(qs.starts_with("qs_"));
        assert_eq!(qs.len(), 3 + 12);
    }

    #[test]
    fn test_session_tokens_generation() {
        let tokens = SessionTokens::new();
        assert!(tokens.chart_session.starts_with("cs_"));
        assert!(tokens.quote_session.starts_with("qs_"));
        assert_ne!(tokens.chart_session, tokens.quote_session);
    }
}
