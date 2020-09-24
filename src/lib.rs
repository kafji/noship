#[cfg(any(debug_assertions, nonoship))]
#[macro_export]
macro_rules! noship {
    () => {
        panic!("not yet implemented")
    };
    ($msg:expr) => {
        panic!($msg)
    };
}

#[cfg(not(any(debug_assertions, nonoship)))]
#[macro_export]
macro_rules! noship {
    () => {
        compile_error!("release blocked")
    };
    ($msg:expr) => {
        compile_error!($msg)
    };
}
