// Copyright 2026 the Subduction Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Layer and surface identity types.

use core::fmt;

/// Sentinel value indicating "no layer" or "no surface" in index fields.
pub const INVALID: u32 = u32::MAX;

/// A handle to a layer in a [`LayerStore`](super::LayerStore).
///
/// Contains both a slot index and a generation counter so that stale handles
/// can be detected after a layer is destroyed and the slot is reused.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayerId {
    /// Slot index into the store's arrays.
    pub(crate) idx: u32,
    /// Generation counter — must match the store's generation for this slot.
    pub(crate) generation: u32,
}

impl LayerId {
    /// Returns the raw slot index (for diagnostics only).
    #[inline]
    #[must_use]
    pub const fn index(self) -> u32 {
        self.idx
    }

    /// Returns the generation counter.
    #[inline]
    #[must_use]
    pub const fn generation(self) -> u32 {
        self.generation
    }
}

impl fmt::Debug for LayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LayerId({}@gen{})", self.idx, self.generation)
    }
}

/// An opaque reference to a content surface.
///
/// Surfaces are created and managed externally (e.g. by an imaging pipeline or
/// GPU backend). A layer with `Some(SurfaceId)` as its content is a leaf that
/// presents that surface; `None` indicates a grouping node.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceId(pub u32);

impl fmt::Debug for SurfaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SurfaceId({})", self.0)
    }
}
