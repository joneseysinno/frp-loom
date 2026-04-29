// ---------------------------------------------------------------------------
// Query
// ---------------------------------------------------------------------------

/// A composable filter + pagination descriptor for store queries.
///
/// Build via the fluent API:
/// ```rust
/// # use frp_loom::query::Query;
/// let q = Query::new()
///     .kind("Atom::Source")
///     .tag("layer:domain")
///     .limit(20)
///     .offset(40);
/// ```
#[derive(Debug, Clone, Default)]
pub struct Query {
    /// If set, only return entities whose `kind` string matches this value.
    pub kind_filter: Option<String>,
    /// All listed tags must be present on the entity (AND semantics).
    pub tag_filter: Vec<String>,
    /// Maximum number of items to return. `None` means no limit.
    pub limit: Option<usize>,
    /// Number of matching items to skip before returning results.
    pub offset: usize,
}

impl Query {
    /// Create a new, unconstrained query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by entity kind (exact match).
    pub fn kind(mut self, k: impl Into<String>) -> Self {
        self.kind_filter = Some(k.into());
        self
    }

    /// Require a specific tag to be present on matching entities.
    /// Multiple calls accumulate (AND semantics).
    pub fn tag(mut self, t: impl Into<String>) -> Self {
        self.tag_filter.push(t.into());
        self
    }

    /// Maximum number of results to return.
    pub fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    /// Number of matching items to skip (for pagination).
    pub fn offset(mut self, n: usize) -> Self {
        self.offset = n;
        self
    }
}

// ---------------------------------------------------------------------------
// QueryResult
// ---------------------------------------------------------------------------

/// The result of a store query, including pagination metadata.
#[derive(Debug, Clone)]
pub struct QueryResult<T> {
    /// The matched items in this page.
    pub items: Vec<T>,
    /// Total number of matching items before pagination was applied.
    pub total: usize,
    /// The offset that was used to produce this page.
    pub offset: usize,
}

impl<T> QueryResult<T> {
    /// Construct a new `QueryResult`.
    pub fn new(items: Vec<T>, total: usize, offset: usize) -> Self {
        Self { items, total, offset }
    }

    /// An empty result with zero total.
    pub fn empty() -> Self {
        Self { items: Vec::new(), total: 0, offset: 0 }
    }

    /// Number of items in this page.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` if no items are in this page.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_builder_sets_fields() {
        let q = Query::new().kind("Source").tag("a").tag("b").limit(10).offset(5);
        assert_eq!(q.kind_filter.as_deref(), Some("Source"));
        assert_eq!(q.tag_filter, vec!["a", "b"]);
        assert_eq!(q.limit, Some(10));
        assert_eq!(q.offset, 5);
    }

    #[test]
    fn query_default_is_unconstrained() {
        let q = Query::new();
        assert!(q.kind_filter.is_none());
        assert!(q.tag_filter.is_empty());
        assert!(q.limit.is_none());
        assert_eq!(q.offset, 0);
    }

    #[test]
    fn query_result_len_and_empty() {
        let r: QueryResult<i32> = QueryResult::new(vec![1, 2, 3], 10, 0);
        assert_eq!(r.len(), 3);
        assert!(!r.is_empty());

        let e: QueryResult<i32> = QueryResult::empty();
        assert!(e.is_empty());
    }
}
