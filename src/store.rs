use plexus_core::{AtomId, BlockId, EdgeId};

use crate::error::StoreError;
use crate::query::{Query, QueryResult};

// ---------------------------------------------------------------------------
// AtomStore
// ---------------------------------------------------------------------------

/// Persistent storage contract for [`Atom`](loom_domain::atom::Atom)-like
/// values, keyed by [`AtomId`].
///
/// Implementors provide a concrete `Atom` associated type (typically the
/// domain `Atom` struct) so that the trait remains generic across multiple
/// storage back-ends without boxing.
pub trait AtomStore {
    /// The atom value type stored and retrieved by this implementation.
    type Atom;

    /// Retrieve an atom by its ID.
    fn get_atom(&self, id: AtomId) -> Result<&Self::Atom, StoreError>;

    /// Insert or replace an atom. The atom's ID is used as the key.
    fn put_atom(&mut self, atom: Self::Atom) -> Result<(), StoreError>;

    /// Delete an atom by ID. Returns `StoreError::NotFound` if absent.
    fn delete_atom(&mut self, id: AtomId) -> Result<(), StoreError>;

    /// Return a paged, optionally-filtered collection of atoms.
    fn query_atoms(&self, query: &Query) -> Result<QueryResult<&Self::Atom>, StoreError>;
}

// ---------------------------------------------------------------------------
// BlockStore
// ---------------------------------------------------------------------------

/// Persistent storage contract for [`Block`](loom_domain::block::Block)-like
/// values, keyed by [`BlockId`].
pub trait BlockStore {
    /// The block value type stored and retrieved by this implementation.
    type Block;

    /// Retrieve a block by its ID.
    fn get_block(&self, id: BlockId) -> Result<&Self::Block, StoreError>;

    /// Insert or replace a block.
    fn put_block(&mut self, block: Self::Block) -> Result<(), StoreError>;

    /// Delete a block by ID. Returns `StoreError::NotFound` if absent.
    fn delete_block(&mut self, id: BlockId) -> Result<(), StoreError>;

    /// Return a paged, optionally-filtered collection of blocks.
    fn query_blocks(&self, query: &Query) -> Result<QueryResult<&Self::Block>, StoreError>;
}

// ---------------------------------------------------------------------------
// EdgeStore
// ---------------------------------------------------------------------------

/// Persistent storage contract for
/// [`HyperEdge`](loom_domain::edge::HyperEdge)-like values, keyed by
/// [`EdgeId`].
pub trait EdgeStore {
    /// The edge value type stored and retrieved by this implementation.
    type Edge;

    /// Retrieve an edge by its ID.
    fn get_edge(&self, id: EdgeId) -> Result<&Self::Edge, StoreError>;

    /// Insert or replace an edge.
    fn put_edge(&mut self, edge: Self::Edge) -> Result<(), StoreError>;

    /// Delete an edge by ID. Returns `StoreError::NotFound` if absent.
    fn delete_edge(&mut self, id: EdgeId) -> Result<(), StoreError>;

    /// Return a paged, optionally-filtered collection of edges.
    fn query_edges(&self, query: &Query) -> Result<QueryResult<&Self::Edge>, StoreError>;
}
