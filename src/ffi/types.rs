use std::os::raw::c_char;

/// C-compatible representation of an OHLCV candle.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FfiCandle {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub has_volume: i32,
}

/// C-compatible representation of a Level 1 quote snapshot.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FfiQuote {
    pub last_price: f64,
    pub change: f64,
    pub change_pct: f64,
    pub bid: f64,
    pub ask: f64,
    pub volume: f64,
}

/// C-compatible representation of a cross-venue arbitrage spread.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FfiArbitrage {
    pub highest_price: f64,
    pub lowest_price: f64,
    pub spread_pct: f64,
}

/// Safe helper to convert a null-terminated C string into a Rust string slice.
///
/// # Safety
/// The pointer must be valid, null-terminated, and non-null.
pub unsafe fn c_str_to_str<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    std::ffi::CStr::from_ptr(ptr).to_str().ok()
}
