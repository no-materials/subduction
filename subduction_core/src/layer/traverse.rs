// Copyright 2026 the Subduction Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tree traversal utilities.

use super::id::{INVALID, LayerId};
use super::store::LayerStore;

/// An iterator over the direct children of a layer.
///
/// Created by [`LayerStore::children`].
#[derive(Debug)]
pub struct Children<'a> {
    store: &'a LayerStore,
    current: u32,
}

impl<'a> Children<'a> {
    pub(crate) fn new(store: &'a LayerStore, first: u32) -> Self {
        Self {
            store,
            current: first,
        }
    }
}

impl Iterator for Children<'_> {
    type Item = LayerId;

    fn next(&mut self) -> Option<LayerId> {
        if self.current == INVALID {
            return None;
        }
        let idx = self.current;
        self.current = self.store.next_sibling[idx as usize];
        Some(LayerId {
            idx,
            generation: self.store.generation[idx as usize],
        })
    }
}
