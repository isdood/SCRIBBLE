use spark_std::glitch::{Glitch, GlitchKind, CrystalError, CrystalErrorExt};
use std::io;

#[test]
fn test_glitch_creation() {
    let err = Glitch::new(GlitchKind::Io, "test error");
    assert_eq!(err.kind(), GlitchKind::Io);
    assert_eq!(err.message(), "test error");
}

#[test]
fn test_glitch_display() {
    let err = Glitch::io("test error");
    assert_eq!(err.to_string(), "test error");
}

#[test]
fn test_glitch_conversion() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = io_err.into_glitch();
    assert_eq!(err.kind(), GlitchKind::Io);
    assert!(err.source().is_some());
}

#[test]
fn test_result_conversion() {
    let result: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::NotFound, "not found"));
    let crystal_result = result.into_crystal();
    assert!(crystal_result.is_err());
}

#[test]
fn test_glitch_kinds() {
    let io = Glitch::io("io error");
    let parse = Glitch::parse("parse error");
    let validation = Glitch::validation("validation error");
    let config = Glitch::config("config error");
    let alignment = Glitch::alignment("alignment error");
    let memory = Glitch::memory("memory error");
    let system = Glitch::system("system error");

    assert_eq!(io.kind(), GlitchKind::Io);
    assert_eq!(parse.kind(), GlitchKind::Parse);
    assert_eq!(validation.kind(), GlitchKind::Validation);
    assert_eq!(config.kind(), GlitchKind::Config);
    assert_eq!(alignment.kind(), GlitchKind::Alignment);
    assert_eq!(memory.kind(), GlitchKind::Memory);
    assert_eq!(system.kind(), GlitchKind::System);
}

#[test]
fn test_backtrace() {
    let err = Glitch::io("test error");
    assert!(err.backtrace().is_some());
}
