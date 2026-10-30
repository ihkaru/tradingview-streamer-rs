use std::ffi::{CStr, CString};
use tradingview_streamer::ffi::{
    tv_streamer_download_sync, tv_streamer_quote_sync, tv_streamer_version, FfiCandle, FfiQuote,
};

#[test]
fn test_c_ffi_version_string() {
    let ver_ptr = tv_streamer_version();
    assert!(!ver_ptr.is_null());
    let c_str = unsafe { CStr::from_ptr(ver_ptr) };
    assert_eq!(c_str.to_str().unwrap(), env!("CARGO_PKG_VERSION"));
}

#[test]
fn test_c_ffi_quote_live() {
    let sym = CString::new("AAPL").unwrap();
    let mut out_quote = FfiQuote {
        last_price: 0.0,
        change: 0.0,
        change_pct: 0.0,
        bid: 0.0,
        ask: 0.0,
        volume: 0.0,
    };

    let status = unsafe { tv_streamer_quote_sync(sym.as_ptr(), &mut out_quote) };
    assert_eq!(status, 0, "C-ABI quote fetch should succeed");
    assert!(out_quote.last_price > 0.0, "Last price must be positive");
}

#[test]
fn test_c_ffi_download_live() {
    let sym = CString::new("AAPL").unwrap();
    let mut buffer = vec![
        FfiCandle {
            timestamp: 0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0.0,
            has_volume: 0,
        };
        20
    ];

    let count = unsafe {
        tv_streamer_download_sync(sym.as_ptr(), buffer.as_mut_ptr(), buffer.len())
    };
    assert!(count > 0, "C-ABI download should write candles into buffer");
    assert!(buffer[0].close > 0.0, "First candle close must be positive");
    assert!(buffer[0].timestamp > 0, "Timestamp must be valid");
}

#[test]
fn test_c_ffi_null_safety() {
    let mut out_quote = FfiQuote {
        last_price: 0.0,
        change: 0.0,
        change_pct: 0.0,
        bid: 0.0,
        ask: 0.0,
        volume: 0.0,
    };

    // Passing null pointer should fail gracefully without crashing
    let res = unsafe { tv_streamer_quote_sync(std::ptr::null(), &mut out_quote) };
    assert_eq!(res, -1);

    let res_dl = unsafe { tv_streamer_download_sync(std::ptr::null(), std::ptr::null_mut(), 10) };
    assert_eq!(res_dl, 0);
}
