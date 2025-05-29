mod domain;

pub mod dbg {
    pub use crate::domain::dbg::Dbg;
}

pub mod error {
    pub use super::domain::error::Error as Error;
}

pub mod log {
    pub use logging::*;
}
#[cfg(test)]
mod tests;