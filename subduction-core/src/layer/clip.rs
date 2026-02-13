// Copyright 2026 the Subduction Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Clip shape types for layer clipping.

/// A shape used to clip a layer's content and descendants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClipShape {
    /// An axis-aligned rectangle.
    Rect(kurbo::Rect),
    /// A rectangle with rounded corners.
    RoundedRect(kurbo::RoundedRect),
}
