// Copyright 2026 the Subduction Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Display output identification.
//!
//! [`OutputId`] is a lightweight handle identifying a specific display or
//! output surface. Backends assign these; core treats them as opaque.

use core::fmt;

/// Identifies a specific display output or surface.
///
/// Backends assign output IDs to distinguish multiple displays. Core code
/// passes them through without interpreting the value.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct OutputId(pub u32);

impl fmt::Debug for OutputId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OutputId({})", self.0)
    }
}
