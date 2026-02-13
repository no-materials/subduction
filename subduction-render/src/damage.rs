// Copyright 2026 the Subduction Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Spatial damage tracking for partial re-rendering.

use alloc::vec::Vec;

/// A region of the output that needs re-rendering.
///
/// Backends can use this to minimize GPU work by only redrawing areas
/// that changed since the last frame.
#[derive(Clone, Debug, Default)]
pub enum DamageRegion {
    /// The entire output needs redrawing.
    #[default]
    Full,
    /// A list of axis-aligned rectangles that need redrawing.
    ///
    /// Each rectangle is `[x, y, width, height]` in output-space pixels.
    Rects(Vec<[f32; 4]>),
    /// Nothing changed; the previous frame can be reused.
    None,
}

impl DamageRegion {
    /// Returns `true` if no region needs redrawing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Merges another damage region into this one.
    pub fn merge(&mut self, other: &Self) {
        match (&*self, other) {
            (Self::Full, _) | (_, Self::Full) => *self = Self::Full,
            (Self::None, _) => *self = other.clone(),
            (_, Self::None) => {}
            (Self::Rects(a), Self::Rects(b)) => {
                let mut merged = a.clone();
                merged.extend_from_slice(b);
                *self = Self::Rects(merged);
            }
        }
    }
}
