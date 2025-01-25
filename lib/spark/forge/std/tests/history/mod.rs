use spark_std::history::{CrystalTrace, Frame, Symbol};

#[test]
fn test_trace_capture() {
    let trace = CrystalTrace::capture();
    assert!(!trace.is_empty());
    assert!(trace.len() > 0);
    assert!(!trace.has_system_frames());
}

#[test]
fn test_trace_with_system() {
    let trace = CrystalTrace::with_system();
    assert!(!trace.is_empty());
    assert!(trace.len() > 0);
    assert!(trace.has_system_frames());
}

#[test]
fn test_frame_resolution() {
    let mut trace = CrystalTrace::capture();

    // Check unresolved state
    let frame = &trace.frames()[0];
    assert!(!frame.is_resolved());
    assert!(frame.ip() != std::ptr::null_mut());
    assert!(frame.symbol_address() != std::ptr::null_mut());

    // Resolve and check
    trace.resolve();
    let frame = &trace.frames()[0];
    assert!(frame.is_resolved());

    if let Some(symbol) = frame.symbol() {
        // At least one of these should be present
        assert!(symbol.name().is_some() || symbol.filename().is_some());
    }
}

#[test]
fn test_trace_display() {
    let mut trace = CrystalTrace::capture();
    trace.resolve();

    let output = format!("{}", trace);
    assert!(output.starts_with("Stack backtrace:"));
    assert!(output.contains("   0: "));
}

#[test]
fn test_frame_display() {
    let mut trace = CrystalTrace::capture();
    trace.resolve();

    let frame = &trace.frames()[0];
    let output = format!("{}", frame);
    assert!(!output.is_empty());
    assert!(output != "<unresolved>");
}
