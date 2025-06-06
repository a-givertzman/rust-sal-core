mod domain;

pub mod dbg {
    pub use crate::domain::dbg::Dbg;
    pub use sal_core_macros::dbg;
    pub use sal_core_macros::debug;
    pub use sal_core_macros::info;
    pub use sal_core_macros::trace;
    pub use sal_core_macros::warn;
    pub use sal_core_macros::error;
}

pub mod error {
    pub use super::domain::error::Error as Error;
    pub use sal_core_macros::err;
    pub use sal_core_macros::err_new;
    pub use sal_core_macros::err_pass;
}

#[cfg(test)]
mod tests;