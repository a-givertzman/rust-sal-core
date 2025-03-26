mod domain;

pub mod error {
    pub use super::domain::error::Error as Error;
}

#[cfg(test)]
mod tests;