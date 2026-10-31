//! Calendar date range and financial period utilities mirroring `yfinance` conventions.

use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::{Error, Result};

/// Closed interval `[start_timestamp, end_timestamp]` in epoch seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    /// Start epoch timestamp in seconds.
    pub start_timestamp: i64,
    /// End epoch timestamp in seconds.
    pub end_timestamp: i64,
}

impl DateRange {
    /// Constructs a DateRange from explicit epoch timestamps in seconds.
    pub fn from_timestamps(start: i64, end: i64) -> Result<Self> {
        if start > end {
            return Err(Error::InvalidParameter(format!(
                "Invalid date range: start timestamp ({start}) cannot be after end timestamp ({end})"
            )));
        }
        Ok(Self {
            start_timestamp: start,
            end_timestamp: end,
        })
    }

    /// Parses start and end date strings in `"YYYY-MM-DD"` format (e.g. `"2025-01-01"`).
    pub fn parse_dates(start_date: &str, end_date: &str) -> Result<Self> {
        let start = parse_ymd_to_timestamp(start_date)?;
        let end = parse_ymd_to_timestamp(end_date)? + 86_399; // Include the full end day
        Self::from_timestamps(start, end)
    }

    /// Checks if a candle timestamp falls within this date range.
    #[inline]
    pub fn contains(&self, timestamp: i64) -> bool {
        timestamp >= self.start_timestamp && timestamp <= self.end_timestamp
    }
}

/// Standard financial period window mirroring yfinance period specifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Period {
    /// Number of days (e.g. "1d", "5d").
    Days(u32),
    /// Number of months (e.g. "1mo", "3mo", "6mo").
    Months(u32),
    /// Number of years (e.g. "1y", "2y", "5y").
    Years(u32),
    /// Year-to-date starting from January 1st of current year.
    YearToDate,
    /// Maximum available historical dataset.
    Max,
}

impl FromStr for Period {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let trimmed = s.trim().to_lowercase();
        if trimmed == "ytd" {
            return Ok(Self::YearToDate);
        }
        if trimmed == "max" {
            return Ok(Self::Max);
        }

        if let Some(num_str) = trimmed.strip_suffix("mo") {
            let n: u32 = num_str.parse().map_err(|_| Error::InvalidParameter(format!("Invalid period '{s}'")))?;
            return Ok(Self::Months(n));
        }
        if let Some(num_str) = trimmed.strip_suffix('d') {
            let n: u32 = num_str.parse().map_err(|_| Error::InvalidParameter(format!("Invalid period '{s}'")))?;
            return Ok(Self::Days(n));
        }
        if let Some(num_str) = trimmed.strip_suffix('y') {
            let n: u32 = num_str.parse().map_err(|_| Error::InvalidParameter(format!("Invalid period '{s}'")))?;
            return Ok(Self::Years(n));
        }

        Err(Error::InvalidParameter(format!(
            "Unsupported period format '{s}'. Expected: '1d', '5d', '1mo', '3mo', '6mo', '1y', 'ytd', 'max'"
        )))
    }
}

impl Period {
    /// Calculates the start and end epoch timestamps in seconds relative to current time.
    pub fn to_timestamps(&self, now_ts: i64) -> (i64, i64) {
        let start_ts = match self {
            Self::Days(d) => now_ts - (*d as i64 * 86_400),
            Self::Months(m) => now_ts - (*m as i64 * 30 * 86_400),
            Self::Years(y) => now_ts - (*y as i64 * 365 * 86_400),
            Self::YearToDate => {
                let sec_in_year = now_ts % 31_557_600;
                now_ts - sec_in_year
            }
            Self::Max => 0,
        };
        (start_ts.max(0), now_ts)
    }

    /// Converts this period into a [`DateRange`] relative to current system time.
    pub fn to_date_range(&self) -> DateRange {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let (start, end) = self.to_timestamps(now);
        DateRange {
            start_timestamp: start,
            end_timestamp: end,
        }
    }
}

/// Helper parsing "YYYY-MM-DD" into epoch seconds timestamp.
pub fn parse_ymd_to_timestamp(date_str: &str) -> Result<i64> {
    let parts: Vec<&str> = date_str.trim().split('-').collect();
    if parts.len() != 3 || parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
        return Err(Error::InvalidParameter(format!(
            "Invalid date format '{date_str}'. Expected 'YYYY-MM-DD'"
        )));
    }
    let y: i64 = parts[0].parse().map_err(|_| Error::InvalidParameter(date_str.to_string()))?;
    let m: i64 = parts[1].parse().map_err(|_| Error::InvalidParameter(date_str.to_string()))?;
    let d: i64 = parts[2].parse().map_err(|_| Error::InvalidParameter(date_str.to_string()))?;

    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return Err(Error::InvalidParameter(format!("Invalid calendar date '{date_str}'")));
    }

    let mut total_days = (y - 1970) * 365 + ((y - 1969) / 4) - ((y - 1901) / 100) + ((y - 1601) / 400);
    const DAYS_BEFORE_MONTH: [i64; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    total_days += DAYS_BEFORE_MONTH[(m - 1) as usize];
    let is_leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
    if is_leap && m > 2 {
        total_days += 1;
    }
    total_days += d - 1;

    Ok(total_days * 86_400)
}
