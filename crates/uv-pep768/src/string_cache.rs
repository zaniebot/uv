//! Intern Python `str` payloads (filenames, qualnames) across samples.
//!
//! A profiling session typically reads the same handful of filenames
//! thousands of times; deduplicating into [`Arc<str>`] keeps the cost of
//! repeated reads bounded.

use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub(crate) struct StringCache {
    /// Cache keyed by the absolute address of the [`PyUnicodeObject`] in
    /// the target's address space. CPython interns short strings, and
    /// constants like file names and qualnames are reused across many
    /// frames, so this hits often in practice.
    by_addr: HashMap<u64, Arc<str>>,
}

impl StringCache {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    #[expect(dead_code, reason = "consumed once frame walking is fully implemented")]
    pub(crate) fn get_or_insert<F>(&mut self, addr: u64, fetch: F) -> std::io::Result<Arc<str>>
    where
        F: FnOnce() -> std::io::Result<String>,
    {
        if let Some(existing) = self.by_addr.get(&addr) {
            return Ok(Arc::clone(existing));
        }
        let owned = fetch()?;
        let interned: Arc<str> = Arc::from(owned);
        self.by_addr.insert(addr, Arc::clone(&interned));
        Ok(interned)
    }
}
