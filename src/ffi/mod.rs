pub mod types;

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

pub use types::{c_str_to_str, FfiArbitrage, FfiCandle, FfiQuote};

/// Returns the static crate version string as a null-terminated C string.
#[no_mangle]
pub extern "C" fn tv_streamer_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}

/// Synchronously downloads daily OHLCV candles for a symbol via C-ABI.
/// Returns the number of candles written into `out_candles` (up to `max_count`).
///
/// # Safety
/// Pointers must be valid and non-null. `out_candles` must point to an array of at least `max_count` elements.
#[no_mangle]
pub unsafe extern "C" fn tv_streamer_download_sync(
    symbol: *const c_char,
    out_candles: *mut FfiCandle,
    max_count: usize,
) -> usize {
    if symbol.is_null() || out_candles.is_null() || max_count == 0 {
        return 0;
    }

    let symbol_str = match c_str_to_str(symbol) {
        Some(s) => s,
        None => return 0,
    };

    let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(r) => r,
        Err(_) => return 0,
    };

    let candles = match rt.block_on(crate::download(symbol_str)) {
        Ok(c) => c,
        Err(_) => return 0,
    };

    let write_count = candles.len().min(max_count);
    for i in 0..write_count {
        let src = &candles[i];
        let dst = out_candles.add(i);
        ptr::write(
            dst,
            FfiCandle {
                timestamp: src.timestamp,
                open: src.open,
                high: src.high,
                low: src.low,
                close: src.close,
                volume: src.volume.unwrap_or(0.0),
                has_volume: if src.volume.is_some() { 1 } else { 0 },
            },
        );
    }

    write_count
}

/// Synchronously fetches the latest quote snapshot for a symbol via C-ABI.
/// Returns 0 on success, or -1 on error.
///
/// # Safety
/// Pointers must be valid and non-null.
#[no_mangle]
pub unsafe extern "C" fn tv_streamer_quote_sync(
    symbol: *const c_char,
    out_quote: *mut FfiQuote,
) -> i32 {
    if symbol.is_null() || out_quote.is_null() {
        return -1;
    }

    let symbol_str = match c_str_to_str(symbol) {
        Some(s) => s,
        None => return -1,
    };

    let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(r) => r,
        Err(_) => return -1,
    };

    let q = match rt.block_on(crate::quote(symbol_str)) {
        Ok(q) => q,
        Err(_) => return -1,
    };

    ptr::write(
        out_quote,
        FfiQuote {
            last_price: q.last_price.unwrap_or(0.0),
            change: q.change.unwrap_or(0.0),
            change_pct: q.change_percentage.unwrap_or(0.0),
            bid: q.bid.unwrap_or(0.0),
            ask: q.ask.unwrap_or(0.0),
            volume: q.volume.unwrap_or(0.0),
        },
    );

    0
}

/// Deallocates a C string allocated by this library.
///
/// # Safety
/// `s` must be a pointer returned by an allocating function from this library.
#[no_mangle]
pub unsafe extern "C" fn tv_streamer_free_string(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}
