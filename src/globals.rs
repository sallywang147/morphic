// src/globals.rs
use std::sync::atomic::{AtomicI64};

/// A global counter tracking the index value.
pub static GLOBAL_INDEX: AtomicI64 = AtomicI64::new(0);
