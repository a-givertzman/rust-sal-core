mod domain;

pub mod dbg {
    pub use crate::domain::dbg::Dbg;
}

pub mod error {
    pub use super::domain::error::Error as Error;
}

#[cfg(test)]
mod tests;