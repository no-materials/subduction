// Copyright 2026 the Subduction Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Android backend for subduction.
//!
//! This crate will provide integration with Android display APIs:
//!
//! - Choreographer vsync callback tick source (estimated timing)
//! - Surface / `ANativeWindow` presenter
//! - Vulkan/GL swapchain management
