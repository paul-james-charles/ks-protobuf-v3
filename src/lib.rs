//! This crate implements Protocol Buffers

#![warn(rustdoc::missing_doc_code_examples)]
#![deny(
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
#![deny(
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

mod buffer;
mod encoding_traits;
mod errors;
mod fixed32_encoding;
mod fixed64_encoding;
mod length_delimited_encoding;
mod tag_encoding;
mod type_encoding;
mod varint_encoding;
mod zigzag_encoding;

pub use buffer::*;
pub use encoding_traits::*;
pub use errors::*;
pub use fixed32_encoding::*;
pub use fixed64_encoding::*;
pub use length_delimited_encoding::*;
pub use tag_encoding::*;
pub use type_encoding::*;
pub use varint_encoding::*;
pub use zigzag_encoding::*;
