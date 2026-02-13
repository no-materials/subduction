// Copyright 2026 the Subduction Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! X11 backend for subduction.
//!
//! This crate will provide integration with X11 display:
//!
//! - Present extension / `GLX_OML_sync_control` for timing when available
//! - Timer-based pacing fallback
//! - Capability detection and graceful degradation
