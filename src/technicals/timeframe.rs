//! Timeframe encoding and resolution for technical indicator queries.

/// Formats timeframe parameter for TradingView indicator query fields.
#[must_use]
pub fn format_indicator_timeframe(tf: &str) -> &'static str {
    match tf.trim().to_ascii_lowercase().as_str() {
        "1m" | "1" => "|1",
        "5m" | "5" => "|5",
        "15m" | "15" => "|15",
        "30m" | "30" => "|30",
        "1h" | "60" | "60m" => "|60",
        "2h" | "120" | "120m" => "|120",
        "4h" | "240" | "240m" => "|240",
        "1d" | "d" | "daily" => "",
        "1w" | "w" | "weekly" => "|1W",
        "1m_month" | "1mth" | "monthly" => "|1M",
        _ => "",
    }
}
