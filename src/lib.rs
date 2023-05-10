#[cfg(target_pointer_width = "64")]
pub use mcore64::*;

#[cfg(target_pointer_width = "32")]
pub use mcore32::*;

