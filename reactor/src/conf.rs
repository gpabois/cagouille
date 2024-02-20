#[cfg(feature = "thread")]
pub mod sync {
    pub use core::marker::{Send as MaybeSend, Sync as MaybeSync};
}
#[cfg(feature = "thread")]
pub use sync::*;

#[cfg(not(feature = "thread"))]
#[macro_use]
pub mod local {

    #[macro_export]
    macro_rules! requirements {
        () => {'static }
    }
}
