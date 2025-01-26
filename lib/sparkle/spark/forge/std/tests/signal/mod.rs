use spark_std::signal::{Signal, SignalResult};
use std::io::Cursor;

#[test]
fn test_signal_creation() {
    let signal = Signal::new();
    assert!(signal.buffer().is_empty());
}

#[test]
fn test_signal_from_bytes() {
    let bytes = [0, 1, 2, 3, 4];
    let signal = Signal::from_bytes(&bytes);
    assert_eq!(signal.buffer(), &bytes);
}

#[test]
fn test_signal_io() -> SignalResult<()> {
    let mut signal = Signal::new();
    let mut cursor = Cursor::new(vec![1, 2, 3, 4, 5]);

    signal.read_tunneled(&mut cursor)?;
    assert!(!signal.buffer().is_empty());

    let mut output = Vec::new();
    signal.write_tunneled(&mut output)?;

    Ok(())
}

#[test]
fn test_signal_resonance() -> SignalResult<()> {
    let mut signal1 = Signal::from_bytes(&[1, 2, 3]);
    let signal2 = Signal::from_bytes(&[4, 5, 6]);

    signal1.resonate(&signal2)?;
    assert_eq!(signal1.buffer().len(), 3);

    Ok(())
}

#[test]
fn test_signal_amplification() -> SignalResult<()> {
    let mut signal = Signal::from_bytes(&[100, 150, 200]);

    signal.amplify(1.5)?;
    assert!(signal.buffer().iter().any(|&b| b > 100));

    Ok(())
}

#[test]
fn test_signal_bitwise() {
    let signal1 = Signal::from_bytes(&[0xF0, 0xF0]);
    let signal2 = Signal::from_bytes(&[0x0F, 0x0F]);

    let and = signal1.clone() & signal2.clone();
    let or = signal1.clone() | signal2.clone();
    let xor = signal1.clone() ^ signal2;
    let not = !signal1;

    assert!(!and.buffer().is_empty());
    assert!(!or.buffer().is_empty());
    assert!(!xor.buffer().is_empty());
    assert!(!not.buffer().is_empty());
}
