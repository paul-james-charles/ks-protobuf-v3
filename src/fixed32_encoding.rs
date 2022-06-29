use crate::{Buffer, DecodeError};

/// Encodes a 32 bit in a raw little endian format
///
/// This fixed32 encoding ensure 32bit values are stored as 4 bytes
///
/// Basic usage:
/// ```
/// use ks_protobuf_v3::{Buffer, encode_fixed32};
///
/// let mut buffer = Buffer::default();
///
/// let encoded_length = encode_fixed32(20, &mut buffer);
/// ```
pub fn encode_fixed32(value: u32, buffer: &mut Buffer) -> usize {
    let bytes = value.to_le_bytes();

    buffer.put_u8(bytes[0]);
    buffer.put_u8(bytes[1]);
    buffer.put_u8(bytes[2]);
    buffer.put_u8(bytes[3]);
    4
}

/// Decodes a Varint to a 32 bit unsigned integer
///
/// Basic usage:
/// ```
/// use ks_protobuf_v3::{Buffer, decode_fixed32};
///
/// let mut buffer: Vec<u8> = vec![1, 0, 0, 0];
///
/// let (value, len) = decode_fixed32(&buffer).unwrap().unwrap();
/// ```
pub fn decode_fixed32(data: &[u8]) -> Result<Option<(u32, usize)>, DecodeError> {
    if data.len() < 4 {
        Ok(None)
    } else {
        let value: u32 = data[0] as u32
            + ((data[1] as u32) << 8)
            + ((data[2] as u32) << 16)
            + ((data[3] as u32) << 24);

        Ok(Some((value, 4)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Buffer;
    use rstest::*;

    #[rstest]
    #[case(1, vec![1, 0, 0, 0])]
    #[case(u8::MIN as u32, vec![0, 0, 0, 0])]
    #[case(u8::MAX as u32, vec![255, 0, 0, 0])]
    #[case(i8::MIN as u32, vec![128, 255, 255, 255])]
    #[case(i8::MAX as u32, vec![127, 0, 0, 0])]
    #[case(u16::MAX as u32, vec![255, 255, 0, 0])]
    #[case(i16::MIN as u32, vec![0, 128, 255, 255])]
    #[case(i16::MAX as u32, vec![255, 127, 0, 0])]
    #[case(u32::MAX, vec![255, 255, 255, 255])]
    #[case(i32::MIN as u32, vec![0, 0, 0, 128])]
    #[case(i32::MAX as u32, vec![255, 255, 255, 127])]
    #[case(f32::MIN as u32, vec![0, 0, 0, 0])]
    #[case(f32::MAX as u32, vec![255, 255, 255, 255])]
    fn test_fixed32_encoding(#[case] value: u32, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = encode_fixed32(value, &mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![1, 0, 0, 0], 1, 4)]
    #[case(vec![0, 0, 0, 0], u8::MIN as u32, 4)]
    #[case(vec![255, 0, 0, 0], u8::MAX as u32, 4)]
    #[case(vec![128, 255, 255, 255], i8::MIN as u32, 4)]
    #[case(vec![127, 0, 0, 0], i8::MAX as u32, 4)]
    #[case(vec![255, 255, 0, 0], u16::MAX as u32, 4)]
    #[case(vec![0, 128, 255, 255], i16::MIN as u32, 4)]
    #[case(vec![255, 127, 0, 0], i16::MAX as u32, 4)]
    #[case(vec![255, 255, 255, 255], u32::MAX, 4)]
    #[case(vec![0, 0, 0, 128], i32::MIN as u32, 4)]
    #[case(vec![255, 255, 255, 127], i32::MAX as u32, 4)]
    #[case(vec![0, 0, 0, 0], f32::MIN as u32, 4)]
    #[case(vec![255, 255, 255, 255], f32::MAX as u32, 4)]
    fn test_fixed32_decoding(
        #[case] bytes: Vec<u8>,
        #[case] expected_value: u32,
        #[case] expected_len: usize,
    ) {
        // Act
        let (value, len) = decode_fixed32(&bytes).unwrap().unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(len, expected_len);
    }

    #[test]
    fn test_fixed32_decoding_empty_buffer() {
        // Arrange
        let bytes = Vec::new();

        // Act
        let result = decode_fixed32(&bytes).unwrap();

        // Assert
        assert_eq!(result, None);
    }
}
