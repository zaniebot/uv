use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use uv_cache_key::{CacheKey, CacheKeyHasher};
use uv_normalize::PackageName;

use crate::Requirement;

/// Lowered extra build dependencies with source resolution applied.
#[derive(Debug, Clone, Default)]
pub struct ExtraBuildRequires(BTreeMap<PackageName, Vec<Requirement>>);

impl std::ops::Deref for ExtraBuildRequires {
    type Target = BTreeMap<PackageName, Vec<Requirement>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ExtraBuildRequires {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for ExtraBuildRequires {
    type Item = (PackageName, Vec<Requirement>);
    type IntoIter = std::collections::btree_map::IntoIter<PackageName, Vec<Requirement>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<(PackageName, Vec<Requirement>)> for ExtraBuildRequires {
    fn from_iter<T: IntoIterator<Item = (PackageName, Vec<Requirement>)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

/// A map of extra build variables, from variable name to value.
pub type BuildVariables = BTreeMap<String, String>;

/// Extra environment variables to set during builds, on a per-package basis.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ExtraBuildVariables(BTreeMap<PackageName, BuildVariables>);

impl std::ops::Deref for ExtraBuildVariables {
    type Target = BTreeMap<PackageName, BuildVariables>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ExtraBuildVariables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for ExtraBuildVariables {
    type Item = (PackageName, BuildVariables);
    type IntoIter = std::collections::btree_map::IntoIter<PackageName, BuildVariables>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<(PackageName, BuildVariables)> for ExtraBuildVariables {
    fn from_iter<T: IntoIterator<Item = (PackageName, BuildVariables)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl CacheKey for ExtraBuildVariables {
    fn cache_key(&self, state: &mut CacheKeyHasher) {
        for (package, vars) in &self.0 {
            package.as_str().cache_key(state);
            for (key, value) in vars {
                key.cache_key(state);
                value.cache_key(state);
            }
        }
    }
}
