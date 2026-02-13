// Copyright 2026 the Subduction Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Recording, pretty-printing, and Chrome trace export for subduction
//! diagnostics.
//!
//! This crate provides [`TraceSink`](subduction_core::trace::TraceSink)
//! implementations for development and post-mortem analysis:
//!
//! - [`pretty::PrettyPrintSink`] — human-readable one-line-per-event output.
//! - [`recorder::RecorderSink`] — compact binary recording with
//!   [`recorder::decode`] for playback.
//! - [`chrome::ChromeTraceExporter`] — writes Chrome Trace Event Format JSON
//!   from recorded bytes.

pub mod chrome;
pub mod pretty;
pub mod recorder;
