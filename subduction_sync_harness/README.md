# subduction_sync_harness

Reusable timing/sync metrics for Subduction demos.

This crate centralizes:

- frame-delta ring-buffer tracking
- hard/soft miss-rate accounting
- capability-aware sync grading (`Predictive`/`Estimated`/`PacingOnly`)
- optional ASCII sparkline generation for HUDs

It is intended for examples and diagnostics (web + macOS), not production
rendering policy.
