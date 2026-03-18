//! USDZ (packaged scene) reader for layerstack.
//!
//! Reads USDZ package files per AOUSD Core §16.4 and produces [`Layer`] /
//! [`PrimSpec`] structures compatible with the layerstack composition engine.
//!
//! USDZ is a constrained ZIP archive containing USD layers and associated
//! media (textures, audio). Constraints (§16.4.1):
//! - All entries are uncompressed (Stored)
//! - No encryption
//! - 32-bit ZIP only (no Zip64)
//! - Local file header offsets are 64-byte aligned
//! - No End of Central Directory comment
//! - First file is the root USD layer
//!
//! The reader operates on a byte slice (`&[u8]`), making it suitable for
//! both file reads and memory-mapped I/O.
//!
//! # Pipeline
//!
//! ```text
//! &[u8] → ZIP parse → USDZ validation → root layer → format dispatch → Layer
//! ```
//!
//! [`Layer`]: layerstack::doc::Layer
//! [`PrimSpec`]: layerstack::doc::PrimSpec

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

pub mod crc32;
pub mod error;
pub mod zip;

pub use error::UsdzError;
