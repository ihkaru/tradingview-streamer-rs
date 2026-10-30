use crate::types::OhlcvCandle;

/// High-performance Structure-of-Arrays (SoA) columnar representation of OHLCV bars.
/// Cache-friendly layout designed for high-throughput vectorized SIMD backtesting and analytics.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColumnarCandles {
    pub timestamps: Vec<i64>,
    pub opens: Vec<f64>,
    pub highs: Vec<f64>,
    pub lows: Vec<f64>,
    pub closes: Vec<f64>,
    pub volumes: Vec<Option<f64>>,
}

impl ColumnarCandles {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            timestamps: Vec::with_capacity(capacity),
            opens: Vec::with_capacity(capacity),
            highs: Vec::with_capacity(capacity),
            lows: Vec::with_capacity(capacity),
            closes: Vec::with_capacity(capacity),
            volumes: Vec::with_capacity(capacity),
        }
    }

    /// Converts a slice of row-based [`OhlcvCandle`] records into columnar SoA layout.
    #[must_use]
    pub fn from_candles(candles: &[OhlcvCandle]) -> Self {
        let mut col = Self::with_capacity(candles.len());
        for c in candles {
            col.push(c);
        }
        col
    }

    /// Appends an individual [`OhlcvCandle`] into the columnar vectors.
    pub fn push(&mut self, candle: &OhlcvCandle) {
        self.timestamps.push(candle.timestamp);
        self.opens.push(candle.open);
        self.highs.push(candle.high);
        self.lows.push(candle.low);
        self.closes.push(candle.close);
        self.volumes.push(candle.volume);
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.timestamps.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.timestamps.is_empty()
    }

    /// Exports the columnar candles into a CSV formatted string.
    #[must_use]
    pub fn to_csv(&self) -> String {
        let mut out = String::with_capacity(self.len() * 48);
        out.push_str("timestamp,open,high,low,close,volume\n");
        for i in 0..self.len() {
            let vol_str = match self.volumes[i] {
                Some(v) => v.to_string(),
                None => String::new(),
            };
            out.push_str(&format!(
                "{},{},{},{},{},{}\n",
                self.timestamps[i],
                self.opens[i],
                self.highs[i],
                self.lows[i],
                self.closes[i],
                vol_str
            ));
        }
        out
    }
}
