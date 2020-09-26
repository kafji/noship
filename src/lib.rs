//! Noship is a macro that acts similar to
//! [todo!](https://doc.rust-lang.org/std/macro.todo.html) but will throw a
//! compilation error when compiled on release profile. Think of this as todo
//! but the compiler will refuse to compile on release mode thus prohibit you
//! to release incomplete code in case you forgot about it.
//!
//! ## Example
//!
//! ```ignore
//! use noship::noship;
//!
//! fn main() {
//!     noship!();
//! }
//! ```
//!
//! If you build the code example above on release profile,
//! `cargo build --release`, the compiler will emit a compilation error.
//! ```
//! error: release blocked
//!  --> src/main.rs
//!   |
//!   |     noship!();
//!   |     ^^^^^^^^^^
//!   |
//! ```
//!
//! On dev profile, omitting the `release` flag, it will compile just fine.
//! But, similar to `todo!`, it will panic when you run it.
//!
//! You can force noship to always behave like on dev profile by adding
//! `nonoship` in the rustc cfg args, i.e.
//! `RUSTFLAGS='--cfg nonoship' cargo build --release`

/// Indicates unfinished code.
///
/// Similar to [todo!](https://doc.rust-lang.org/std/macro.todo.html).
///
/// ## Examples
/// ```ignore
/// noship!();
/// noship!("need impl for edge cases");
/// ```
///
/// See the [module-level documentation](index.html) for more.
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

/// Indicates unfinished code.
///
/// Similar to [todo!](https://doc.rust-lang.org/std/macro.todo.html).
///
/// ## Examples
/// ```ignore
/// noship!();
/// noship!("need impl for edge cases");
/// ```
///
/// See the [module-level documentation](index.html) for more.
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
