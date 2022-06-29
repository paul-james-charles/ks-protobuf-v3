use crate::{encode_varint32, Buffer, DecodeError, Varint};

/// Used in the tag to identify how a field is encoded in Protocol Buffers.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WireType {
    /// Wire type is used to store values using LEB128
    Varint = 0,

    /// Wire type is used to store 64 bit values
    Fixed64 = 1,

    /// Wire type is used to variable length variables. I.e. strings.
    LengthDelimited = 2,

    /// Deprecated
    StartGroup = 3,

    /// Deprecated
    EndGroup = 4,

    /// Wire type is used to store 32 bit values
    Fixed32 = 5,
}

impl TryFrom<u32> for WireType {
    type Error = DecodeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WireType::Varint),
            1 => Ok(WireType::Fixed64),
            2 => Ok(WireType::LengthDelimited),
            3 => Ok(WireType::StartGroup),
            4 => Ok(WireType::EndGroup),
            5 => Ok(WireType::Fixed32),
            _ => Err(DecodeError::UnknownWireType),
        }
    }
}

/// This structure is used to represent a tag
#[derive(Debug)]
pub struct Tag {
    field_number: u32,
    wire_type: WireType,
}

impl Tag {
    /// Function to allocate a tag
    pub fn new(field_number: u32, wire_type: WireType) -> Self {
        Self {
            field_number,
            wire_type,
        }
    }
}

impl Varint for Tag {
    fn to_varint(&self, buffer: &mut Buffer) -> usize {
        let tag = (self.field_number << 3) | (self.wire_type as u32);

        encode_varint32(tag, buffer)
    }

    fn from_varint(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let mut tag: u32 = 0;

        let len = tag.from_varint(buffer)?;

        self.field_number = tag >> 3;
        self.wire_type = WireType::try_from(tag & 0x0007)?;
        Ok(len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(1, WireType::Varint, vec![8])]
    #[case(2, WireType::Fixed32, vec![21])]
    #[case(1000, WireType::LengthDelimited, vec![194, 62])]
    #[case(100000, WireType::Fixed64, vec![129, 234, 48])]
    fn test_tag_encoding(
        #[case] field_number: u32,
        #[case] wire_type: WireType,
        #[case] expected_buffer: Vec<u8>,
    ) {
        // Arrange
        let mut buffer = Buffer::default();
        let tag = Tag::new(field_number, wire_type);

        // Act
        let size = tag.to_varint(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![8], 1, WireType::Varint)]
    #[case(vec![21], 2, WireType::Fixed32)]
    #[case(vec![194, 62], 1000, WireType::LengthDelimited)]
    #[case(vec![129, 234, 48], 100000, WireType::Fixed64)]
    fn test_tag_decoding(
        #[case] buffer: Vec<u8>,
        #[case] expected_field_number: u32,
        #[case] expected_wire_type: WireType,
    ) {
        // Arrange
        let mut tag = Tag::new(0, WireType::Varint);

        // Act
        let result = tag.from_varint(&buffer).unwrap();

        // Assert
        assert_eq!(tag.field_number, expected_field_number);
        assert_eq!(tag.wire_type, expected_wire_type);
        assert_eq!(result, buffer.len());
    }
}
