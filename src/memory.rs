//! In-memory [`AtomStore`], [`BlockStore`], and [`EdgeStore`] implementations.
//!
//! Enabled via the `in-memory` Cargo feature. These are backed by
//! `HashMap` and are suitable for tests, prototyping, and single-process
//! deployments that don't require persistence.

use std::collections::HashMap;

use plexus_core::{AtomId, BlockId, EdgeId};

use crate::error::StoreError;
use crate::query::{Query, QueryResult};
use crate::store::{AtomStore, BlockStore, EdgeStore};

// ---------------------------------------------------------------------------
// Helper: apply limit + offset pagination to an iterator of references
// ---------------------------------------------------------------------------

fn paginate<'a, T>(
    iter: impl Iterator<Item = &'a T>,
    total: usize,
    query: &Query,
) -> QueryResult<&'a T> {
    let paged: Vec<&T> = iter
        .skip(query.offset)
        .take(query.limit.unwrap_or(usize::MAX))
        .collect();
    QueryResult::new(paged, total, query.offset)
}

// ---------------------------------------------------------------------------
// InMemoryAtomStore
// ---------------------------------------------------------------------------

/// A `HashMap`-backed [`AtomStore`] for any `Clone` atom type.
///
/// Kind/tag filtering in [`AtomStore::query_atoms`] is deferred to the
/// domain layer — this implementation returns all atoms subject only to
/// limit/offset pagination.
#[derive(Debug, Default)]
pub struct InMemoryAtomStore<V: Clone> {
    data: HashMap<AtomId, V>,
}

impl<V: Clone> InMemoryAtomStore<V> {
    /// Create an empty store.
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    /// Number of atoms currently stored.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Retrieve the `AtomId` from a value. Implement this on your domain `Atom`
/// type so that [`InMemoryAtomStore::put_atom`] can derive the key.
pub trait HasAtomId {
    fn atom_id(&self) -> AtomId;
}

impl<V: Clone + HasAtomId> AtomStore for InMemoryAtomStore<V> {
    type Atom = V;

    fn get_atom(&self, id: AtomId) -> Result<&V, StoreError> {
        self.data.get(&id).ok_or_else(|| StoreError::not_found(id))
    }

    fn put_atom(&mut self, atom: V) -> Result<(), StoreError> {
        let id = atom.atom_id();
        self.data.insert(id, atom);
        Ok(())
    }

    fn delete_atom(&mut self, id: AtomId) -> Result<(), StoreError> {
        self.data.remove(&id).ok_or_else(|| StoreError::not_found(id))?;
        Ok(())
    }

    fn query_atoms(&self, query: &Query) -> Result<QueryResult<&V>, StoreError> {
        let all: Vec<&V> = self.data.values().collect();
        let total = all.len();
        Ok(paginate(all.into_iter(), total, query))
    }
}

// ---------------------------------------------------------------------------
// InMemoryBlockStore
// ---------------------------------------------------------------------------

/// A `HashMap`-backed [`BlockStore`] for any `Clone` block type.
#[derive(Debug, Default)]
pub struct InMemoryBlockStore<V: Clone> {
    data: HashMap<BlockId, V>,
}

impl<V: Clone> InMemoryBlockStore<V> {
    /// Create an empty store.
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    /// Number of blocks currently stored.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Retrieve the `BlockId` from a value.
pub trait HasBlockId {
    fn block_id(&self) -> BlockId;
}

impl<V: Clone + HasBlockId> BlockStore for InMemoryBlockStore<V> {
    type Block = V;

    fn get_block(&self, id: BlockId) -> Result<&V, StoreError> {
        self.data.get(&id).ok_or_else(|| StoreError::not_found(id))
    }

    fn put_block(&mut self, block: V) -> Result<(), StoreError> {
        let id = block.block_id();
        self.data.insert(id, block);
        Ok(())
    }

    fn delete_block(&mut self, id: BlockId) -> Result<(), StoreError> {
        self.data.remove(&id).ok_or_else(|| StoreError::not_found(id))?;
        Ok(())
    }

    fn query_blocks(&self, query: &Query) -> Result<QueryResult<&V>, StoreError> {
        let all: Vec<&V> = self.data.values().collect();
        let total = all.len();
        Ok(paginate(all.into_iter(), total, query))
    }
}

// ---------------------------------------------------------------------------
// InMemoryEdgeStore
// ---------------------------------------------------------------------------

/// A `HashMap`-backed [`EdgeStore`] for any `Clone` edge type.
#[derive(Debug, Default)]
pub struct InMemoryEdgeStore<V: Clone> {
    data: HashMap<EdgeId, V>,
}

impl<V: Clone> InMemoryEdgeStore<V> {
    /// Create an empty store.
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    /// Number of edges currently stored.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Retrieve the `EdgeId` from a value.
pub trait HasEdgeId {
    fn edge_id(&self) -> EdgeId;
}

impl<V: Clone + HasEdgeId> EdgeStore for InMemoryEdgeStore<V> {
    type Edge = V;

    fn get_edge(&self, id: EdgeId) -> Result<&V, StoreError> {
        self.data.get(&id).ok_or_else(|| StoreError::not_found(id))
    }

    fn put_edge(&mut self, edge: V) -> Result<(), StoreError> {
        let id = edge.edge_id();
        self.data.insert(id, edge);
        Ok(())
    }

    fn delete_edge(&mut self, id: EdgeId) -> Result<(), StoreError> {
        self.data.remove(&id).ok_or_else(|| StoreError::not_found(id))?;
        Ok(())
    }

    fn query_edges(&self, query: &Query) -> Result<QueryResult<&V>, StoreError> {
        let all: Vec<&V> = self.data.values().collect();
        let total = all.len();
        Ok(paginate(all.into_iter(), total, query))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Minimal test types --------------------------------------------------

    #[derive(Debug, Clone, PartialEq)]
    struct TestAtom {
        id: AtomId,
        name: &'static str,
    }
    impl HasAtomId for TestAtom {
        fn atom_id(&self) -> AtomId { self.id }
    }

    #[derive(Debug, Clone, PartialEq)]
    struct TestBlock {
        id: BlockId,
    }
    impl HasBlockId for TestBlock {
        fn block_id(&self) -> BlockId { self.id }
    }

    #[derive(Debug, Clone, PartialEq)]
    struct TestEdge {
        id: EdgeId,
    }
    impl HasEdgeId for TestEdge {
        fn edge_id(&self) -> EdgeId { self.id }
    }

    // AtomStore tests -------------------------------------------------------

    #[test]
    fn atom_put_and_get() {
        let mut store = InMemoryAtomStore::new();
        let id = AtomId::new(1);
        store.put_atom(TestAtom { id, name: "a" }).unwrap();
        assert_eq!(store.get_atom(id).unwrap().name, "a");
    }

    #[test]
    fn atom_get_missing_returns_not_found() {
        let store: InMemoryAtomStore<TestAtom> = InMemoryAtomStore::new();
        let err = store.get_atom(AtomId::new(99)).unwrap_err();
        assert!(matches!(err, StoreError::NotFound { .. }));
    }

    #[test]
    fn atom_delete_removes_entry() {
        let mut store = InMemoryAtomStore::new();
        let id = AtomId::new(1);
        store.put_atom(TestAtom { id, name: "x" }).unwrap();
        store.delete_atom(id).unwrap();
        assert!(store.get_atom(id).is_err());
    }

    #[test]
    fn atom_delete_missing_returns_not_found() {
        let mut store: InMemoryAtomStore<TestAtom> = InMemoryAtomStore::new();
        assert!(matches!(
            store.delete_atom(AtomId::new(5)).unwrap_err(),
            StoreError::NotFound { .. }
        ));
    }

    #[test]
    fn atom_query_pagination() {
        let mut store = InMemoryAtomStore::new();
        for i in 0..5u64 {
            store.put_atom(TestAtom { id: AtomId::new(i), name: "x" }).unwrap();
        }
        let q = Query::new().limit(2).offset(1);
        let result = store.query_atoms(&q).unwrap();
        assert_eq!(result.total, 5);
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.offset, 1);
    }

    // BlockStore tests ------------------------------------------------------

    #[test]
    fn block_put_get_delete() {
        let mut store = InMemoryBlockStore::new();
        let id = BlockId::new(10);
        store.put_block(TestBlock { id }).unwrap();
        assert_eq!(store.get_block(id).unwrap().id, id);
        store.delete_block(id).unwrap();
        assert!(store.get_block(id).is_err());
    }

    // EdgeStore tests -------------------------------------------------------

    #[test]
    fn edge_put_get_delete() {
        let mut store = InMemoryEdgeStore::new();
        let id = EdgeId::new(20);
        store.put_edge(TestEdge { id }).unwrap();
        assert_eq!(store.get_edge(id).unwrap().id, id);
        store.delete_edge(id).unwrap();
        assert!(store.get_edge(id).is_err());
    }
}
