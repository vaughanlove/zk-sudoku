#![cfg_attr(not(feature = "std"), no_std)]
pub mod core;

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        // No-op in no_std or custom implementation
    };
}
