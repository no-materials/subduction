// Copyright 2026 the Subduction Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Windows backend for subduction.
//!
//! This crate will provide integration with Windows display and compositing APIs:
//!
//! - QPC-based timing with DXGI present statistics
//! - `DirectComposition` visual tree presenter
//! - DXGI swapchain management
