//! Cretonne code generation library.

#![deny(missing_docs)]

pub use context::Context;
pub use legalizer::legalize_function;
pub use verifier::verify_function;
pub use write::write_function;

/// Version number of the cretonne crate.
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub mod cfg;
pub mod dominator_tree;
pub mod entity_list;
pub mod entity_map;
pub mod ir;
pub mod isa;
pub mod regalloc;
pub mod settings;
pub mod sparse_map;
pub mod verifier;

mod constant_hash;
mod context;
mod legalizer;
mod packed_option;
mod partition_slice;
mod predicates;
mod ref_slice;
mod write;
