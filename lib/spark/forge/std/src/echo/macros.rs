/// Creates a new formatted string
#[macro_export]
macro_rules! echo {
    ($s:literal) => {
        $crate::echo::CrystalEcho::new($s)
    };
    ($($arg:tt)*) => {
        $crate::echo::CrystalEcho::owned(format!($($arg)*))
    };
}
