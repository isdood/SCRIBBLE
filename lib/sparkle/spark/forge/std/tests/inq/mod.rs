use spark_std::inq::{CrystalEnv, CrystalSystem};
use spark_std::inq::platform::{Architecture, OperatingSystem, PlatformFeatures};

#[test]
fn test_env_vars() {
    let env = CrystalEnv::new();
    assert!(env.vars().count() > 0);
}

#[test]
fn test_env_operations() {
    let mut env = CrystalEnv::new();
    env.set("TEST_VAR", "test_value");
    assert_eq!(env.get("TEST_VAR").map(|s| s.to_string_lossy()), Some("test_value".into()));

    env.remove("TEST_VAR");
    assert!(env.get("TEST_VAR").is_none());
}

#[test]
fn test_current_dir() {
    assert!(CrystalEnv::current_dir().is_ok());
}

#[test]
fn test_executable() {
    assert!(CrystalEnv::executable().is_ok());
}

#[test]
fn test_system_info() {
    let sys = CrystalSystem::new();
    assert!(sys.cpu_count() > 0);
}

#[test]
fn test_platform_features() {
    let platform = PlatformFeatures::new();
    assert!(matches!(
        platform.architecture(),
        Architecture::X86_64 | Architecture::AARCH64 | Architecture::X86 | Architecture::ARM | Architecture::RISCV | Architecture::Other
    ));
}

#[test]
fn test_operating_system() {
    let os = OperatingSystem::current();
    assert!(matches!(
        os,
        OperatingSystem::Linux | OperatingSystem::Windows | OperatingSystem::MacOS | OperatingSystem::Other
    ));
}

#[test]
fn test_memory_info() {
    let sys = CrystalSystem::new();
    let mem = sys.memory_info();
    assert!(mem.total() >= mem.used());
}

#[test]
fn test_os_info() {
    let sys = CrystalSystem::new();
    let os = sys.os_info();
    assert!(!os.name().is_empty());
    assert!(!os.version().is_empty());
    assert!(!os.arch().is_empty());
}
