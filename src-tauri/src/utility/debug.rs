/// Println! macro that only prints if debug_assertions are enabled
#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!($($arg)*);
        }
    };
}
