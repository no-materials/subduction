// Copyright 2026 the Subduction Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Opaque resource keys for backend-managed resources.

use core::fmt;

/// An opaque handle to a backend-managed resource (texture, buffer, etc.).
///
/// Resource keys are assigned by backends and passed through the render
/// plan without interpretation by core or render crates.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceKey(pub u64);

impl fmt::Debug for ResourceKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ResourceKey({})", self.0)
    }
}
