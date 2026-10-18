//! `noise-stub` — Handl stdlib port.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub const NOISE_STUB_HNL: &str = include_str!("noise-stub.hnl");
