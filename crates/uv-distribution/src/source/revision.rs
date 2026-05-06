use serde::{Deserialize, Serialize};
use std::path::Path;
use uv_distribution_types::Hashed;

use uv_pypi_types::{HashDigest, HashDigests};

/// The [`Revision`] is a thin wrapper around a unique identifier for the source distribution.
///
/// A revision represents a unique version of a source distribution, at a level more granular than
/// (e.g.) the version number of the distribution itself. For example, a source distribution hosted
/// at a URL or a local file path may have multiple revisions, each representing a unique state of
/// the distribution, despite the reported version number remaining the same.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Revision {
    id: RevisionId,
    hashes: HashDigests,
}

impl Revision {
    /// Initialize a new [`Revision`] with a random UUID.
    pub(crate) fn new() -> Self {
        Self {
            id: RevisionId::new(),
            hashes: HashDigests::empty(),
        }
    }

    /// Return the unique ID of the manifest.
    pub(crate) fn id(&self) -> &RevisionId {
        &self.id
    }

    /// Return the computed hashes of the archive.
    pub(crate) fn hashes(&self) -> &[HashDigest] {
        self.hashes.as_slice()
    }

    /// Return the computed hashes of the archive.
    pub(crate) fn into_hashes(self) -> HashDigests {
        self.hashes
    }

    /// Set the computed hashes of the archive.
    #[must_use]
    pub(crate) fn with_hashes(mut self, hashes: HashDigests) -> Self {
        self.hashes = hashes;
        self
    }
}

impl Hashed for Revision {
    fn hashes(&self) -> &[HashDigest] {
        self.hashes.as_slice()
    }
}

/// A unique identifier for a revision of a source distribution.
///
/// Note: this is a newtype around a [`String`] rather than a [`uv_fastid::Id`] so that we
/// can deserialize cache entries written by older uv versions, which used `nanoid`-style
/// 21-character identifiers. New IDs are still generated via [`uv_fastid::insecure`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct RevisionId(String);

impl RevisionId {
    /// Generate a new unique identifier for an archive.
    fn new() -> Self {
        Self(uv_fastid::insecure().to_string())
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RevisionId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<Path> for RevisionId {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensure that revisions written by older uv versions (with 21-character `nanoid`
    /// identifiers) can still be deserialized after the switch to `uv_fastid`.
    #[test]
    fn legacy_id_deserialize() {
        let legacy = Revision {
            id: RevisionId("V1StGXR8_Z5jdHi6B-myT".to_string()),
            hashes: HashDigests::empty(),
        };
        let bytes = rmp_serde::to_vec(&legacy).unwrap();
        let parsed: Revision = rmp_serde::from_slice(&bytes).unwrap();
        assert_eq!(parsed.id().as_str(), "V1StGXR8_Z5jdHi6B-myT");
    }

    /// Round-trip a freshly generated [`Revision`] through `rmp_serde` to guard against
    /// accidental changes to the on-disk format.
    #[test]
    fn current_id_round_trip() {
        let revision = Revision::new();
        let bytes = rmp_serde::to_vec(&revision).unwrap();
        let parsed: Revision = rmp_serde::from_slice(&bytes).unwrap();
        assert_eq!(parsed.id().as_str(), revision.id().as_str());
    }
}
