use crate::{Buffer, DecodeError, Tag, WireType};

/// This trait can be applied to types to enable them to encode and decode value to and from
/// Protocol Buffers with the Varint wire type.
pub trait Varint {
    /// This function takes a buffer and converts the attached type to a Protocol Buffer encoded
    /// with the Varint wire type.
    fn to_varint(&self, buffer: &mut Buffer) -> usize;

    /// This function takes a buffer and extracts the value encoded as Varint wire type back to the
    /// attached type.
    fn from_varint(&mut self, buffer: &[u8]) -> Result<usize, DecodeError>;
}

/// This trait can be applied to types to enable them to encode and decode fields.
///
/// These fields include the field number, the type of wire type encoding and the value of the
/// attached type encoded.
pub trait VarintField: Varint {
    /// This function writes out a varint field to the Protocol Buffer.
    fn to_varint_field(&self, field_number: u32, buffer: &mut Buffer) -> usize {
        let tag = Tag::new(field_number, WireType::Varint);

        let size1 = tag.to_varint(buffer);
        let size2 = self.to_varint(buffer);
        size1 + size2
    }
}

/// This trait can be applied to types to enable them to encode and decode value to and from
/// Protocol Buffers with the Fixed32 wire type.
pub trait Fixed32 {
    /// This function takes a buffer and converts the attached type to a Protocol Buffer encoded
    /// with the Fixed32 wire type.
    fn to_fixed32(&self, buffer: &mut Buffer) -> usize;

    /// This function takes a buffer and extracts the value encoded as Fixed32 wire type back to the
    /// attached type.
    fn from_fixed32(&mut self, buffer: &[u8]) -> Result<usize, DecodeError>;
}

/// This trait can be applied to types to enable them to encode and decode fields.
///
/// These fields include the field number, the type of wire type encoding and the value of the
/// attached type encoded.
pub trait Fixed32Field: Fixed32 {}

/// This trait can be applied to types to enable them to encode and decode value to and from
/// Protocol Buffers with the Fixed32 wire type.
pub trait Fixed64 {
    /// This function takes a buffer and converts the attached type to a Protocol Buffer encoded
    /// with the Fixed64 wire type.
    fn to_fixed64(&self, buffer: &mut Buffer) -> usize;

    /// This function takes a buffer and extracts the value encoded as Fixed64 wire type back to the
    /// attached type.
    fn from_fixed64(&mut self, buffer: &[u8]) -> Result<usize, DecodeError>;
}

/// This trait can be applied to types to enable them to encode and decode fields.
///
/// These fields include the field number, the type of wire type encoding and the value of the
/// attached type encoded.
pub trait Fixed64Field: Fixed64 {}

/// This trait can be applied to types to enable them to encode and decode value to and from
/// Protocol Buffers with the Length Delimited wire type.
pub trait LengthDelimited {
    /// This function takes a buffer and converts the attached type to a Protocol Buffer encoded
    /// with the Length Delimited wire type.
    fn to_length_delimited(&self, buffer: &mut Buffer) -> usize;

    /// This function takes a buffer and extracts the value encoded as Length Delimited wire type
    /// back to the attached type.
    fn from_length_delimited(&mut self, buffer: &[u8]) -> Result<usize, DecodeError>;
}

/// This trait can be applied to types to enable them to encode and decode fields.
///
/// These fields include the field number, the type of wire type encoding and the value of the
/// attached type encoded.
pub trait LengthDelimitedField: LengthDelimited {}
