//! Core utilities shared across RDKit Rust crates.

use thiserror::Error;

/// Generic error type for the RDKit Rust rewrite.
#[derive(Debug, Error)]
pub enum RdError {
    #[error("Invariant failed: {0}")]
    InvariantFailed(&'static str),

    #[error("Unimplemented feature: {0}")]
    Unimplemented(&'static str),
}

/// Helper type alias.
pub type Result<T, E = RdError> = std::result::Result<T, E>;

// ---------------------------------------------------------------------------
// Sub-modules
// ---------------------------------------------------------------------------

pub mod mol;

/// Runtime invariant check (similar to RDKit's `PRECONDITION`).
#[macro_export]
macro_rules! invariant {
    ($cond:expr, $msg:literal) => {
        if !$cond {
            return Err($crate::RdError::InvariantFailed($msg));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invariant_macro() {
        let ok: Result<()> = (|| {
            invariant!(1 + 1 == 2, "math broke");
            Ok(())
        })();

        assert!(ok.is_ok());

        let err: Result<()> = (|| {
            invariant!(false, "bad");
            Ok(())
        })();

        assert!(matches!(err, Err(RdError::InvariantFailed("bad"))));
    }
}
