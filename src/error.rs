use thiserror::Error;

/// Errors returned by [`AtomStore`](crate::store::AtomStore),
/// [`BlockStore`](crate::store::BlockStore), and
/// [`EdgeStore`](crate::store::EdgeStore) implementations.
#[derive(Debug, Error)]
pub enum StoreError {
    /// An entity with the given ID could not be found.
    #[error("not found: {id}")]
    NotFound { id: String },

    /// A write was rejected due to a duplicate key or failed optimistic
    /// concurrency check.
    #[error("conflict on {id}: {reason}")]
    Conflict { id: String, reason: String },

    /// An I/O or serialization failure occurred.
    #[error("io error: {0}")]
    Io(String),

    /// The supplied [`Query`](crate::query::Query) was malformed.
    #[error("invalid query: {0}")]
    InvalidQuery(String),
}

impl StoreError {
    /// Convenience constructor for `NotFound`.
    pub fn not_found(id: impl std::fmt::Display) -> Self {
        Self::NotFound { id: id.to_string() }
    }

    /// Convenience constructor for `Conflict`.
    pub fn conflict(id: impl std::fmt::Display, reason: impl Into<String>) -> Self {
        Self::Conflict { id: id.to_string(), reason: reason.into() }
    }

    /// Convenience constructor for `Io`.
    pub fn io(msg: impl Into<String>) -> Self {
        Self::Io(msg.into())
    }

    /// Convenience constructor for `InvalidQuery`.
    pub fn invalid_query(msg: impl Into<String>) -> Self {
        Self::InvalidQuery(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_found_display() {
        let e = StoreError::not_found("atom-42");
        assert_eq!(e.to_string(), "not found: atom-42");
    }

    #[test]
    fn conflict_display() {
        let e = StoreError::conflict("block-1", "duplicate key");
        assert_eq!(e.to_string(), "conflict on block-1: duplicate key");
    }
}
