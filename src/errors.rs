use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum DecodeError {
    #[error("The buffer was overrun during decoding.")]
    BufferOverrun,

    #[error("The encoded data is invalid.")]
    UnableToDecode,

    #[error("The decoded value was too large.")]
    UnableToDecodeBecauseTheValueWasTooLarge,

    #[error("The decoded value was too small.")]
    UnableToDecodeBecauseTheValueWasTooSmall,

    #[error("The wire type was unrecognised during decoding.")]
    UnknownWireType,
}
