//! Storage traits and in-memory implementations for the Loom graph layer.
//!
//! Provides `AtomStore`, `BlockStore`, and `EdgeStore` traits plus
//! `HashMap`-backed in-memory implementations (behind the `in-memory` feature
//! flag). Also exports `Query`/`QueryResult` for pagination and `StoreError`.

pub mod error;
pub mod memory;
pub mod query;
pub mod store;

pub use error::StoreError;
pub use memory::{
    HasAtomId, HasBlockId, HasEdgeId, InMemoryAtomStore, InMemoryBlockStore, InMemoryEdgeStore,
};
pub use query::{Query, QueryResult};
pub use store::{AtomStore, BlockStore, EdgeStore};
