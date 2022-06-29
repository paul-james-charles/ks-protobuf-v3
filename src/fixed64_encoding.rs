use crate::{Buffer, DecodeError};

/// Encodes a 64 bit in a raw little endian format
///
/// This fixed64 encoding ensure 64bit values are stored as 4 bytes
///
/// Basic usage:
/// ```
/// use ks_protobuf_v3::{Buffer, encode_fixed64};
///
/// let mut buffer = Buffer::default();
///
/// let encoded_length = encode_fixed64(20, &mut buffer);
/// ```
pub fn encode_fixed64(value: u64, buffer: &mut Buffer) -> usize {
    let bytes = value.to_le_bytes();

    buffer.put_u8(bytes[0]);
    buffer.put_u8(bytes[1]);
    buffer.put_u8(bytes[2]);
    buffer.put_u8(bytes[3]);
    buffer.put_u8(bytes[4]);
    buffer.put_u8(bytes[5]);
    buffer.put_u8(bytes[6]);
    buffer.put_u8(bytes[7]);
    8
}

/// Decodes a Varint to a 64 bit unsigned integer
///
/// Basic usage:
/// ```
/// use ks_protobuf_v3::{Buffer, decode_fixed64};
///
/// let mut buffer: Vec<u8> = vec![1, 0, 0, 0, 0, 0, 0, 0];
///
/// let (value, len) = decode_fixed64(&buffer).unwrap().unwrap();
/// ```
pub fn decode_fixed64(data: &[u8]) -> Result<Option<(u64, usize)>, DecodeError> {
    if data.len() < 4 {
        Ok(None)
    } else {
        let value: u64 = data[0] as u64
            + ((data[1] as u64) << 8)
            + ((data[2] as u64) << 16)
            + ((data[3] as u64) << 24)
            + ((data[4] as u64) << 32)
            + ((data[5] as u64) << 40)
            + ((data[6] as u64) << 48)
            + ((data[7] as u64) << 56);

        Ok(Some((value, 8)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Buffer;
    use rstest::*;

    #[rstest]
    #[case(1, vec![1, 0, 0, 0, 0, 0, 0, 0])]
    #[case(u8::MIN as u64, vec![0, 0, 0, 0, 0, 0, 0, 0])]
    #[case(u8::MAX as u64, vec![255, 0, 0, 0, 0, 0, 0, 0])]
    #[case(i8::MIN as u64, vec![128, 255, 255, 255, 255, 255, 255, 255])]
    #[case(i8::MAX as u64, vec![127, 0, 0, 0, 0, 0, 0, 0])]
    #[case(u16::MAX as u64, vec![255, 255, 0, 0, 0, 0, 0, 0])]
    #[case(i16::MIN as u64, vec![0, 128, 255, 255, 255, 255, 255, 255])]
    #[case(i16::MAX as u64, vec![255, 127, 0, 0, 0, 0, 0, 0])]
    #[case(u32::MAX as u64, vec![255, 255, 255, 255, 0, 0, 0, 0])]
    #[case(i32::MIN as u64, vec![0, 0, 0, 128, 255, 255, 255, 255])]
    #[case(i32::MAX as u64, vec![255, 255, 255, 127, 0, 0, 0, 0])]
    #[case(u64::MAX, vec![255, 255, 255, 255, 255, 255, 255, 255])]
    #[case(i64::MIN as u64, vec![0, 0, 0, 0, 0, 0, 0, 128])]
    #[case(i64::MAX as u64, vec![255, 255, 255, 255, 255, 255, 255, 127])]
    #[case(f64::MIN as u64, vec![0, 0, 0, 0, 0, 0, 0, 0])]
    #[case(f64::MAX as u64, vec![255, 255, 255, 255, 255, 255, 255, 255])]
    fn test_fixed64_encoding(#[case] value: u64, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = encode_fixed64(value, &mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![1, 0, 0, 0, 0, 0, 0, 0], 1, 8)]
    #[case(vec![0, 0, 0, 0, 0, 0, 0, 0], u8::MIN as u64, 8)]
    #[case(vec![255, 0, 0, 0, 0, 0, 0, 0], u8::MAX as u64, 8)]
    #[case(vec![128, 255, 255, 255, 255, 255, 255, 255], i8::MIN as u64, 8)]
    #[case(vec![127, 0, 0, 0, 0, 0, 0, 0], i8::MAX as u64, 8)]
    #[case(vec![255, 255, 0, 0, 0, 0, 0, 0], u16::MAX as u64, 8)]
    #[case(vec![0, 128, 255, 255, 255, 255, 255, 255], i16::MIN as u64, 8)]
    #[case(vec![255, 127, 0, 0, 0, 0, 0, 0], i16::MAX as u64, 8)]
    #[case(vec![255, 255, 255, 255, 0, 0, 0, 0], u32::MAX as u64, 8)]
    #[case(vec![0, 0, 0, 128, 255, 255, 255, 255], i32::MIN as u64, 8)]
    #[case(vec![255, 255, 255, 127, 0, 0, 0, 0], i32::MAX as u64, 8)]
    #[case(vec![255, 255, 255, 255, 255, 255, 255, 255], u64::MAX, 8)]
    #[case(vec![0, 0, 0, 0, 0, 0, 0, 128], i64::MIN as u64, 8)]
    #[case(vec![255, 255, 255, 255, 255, 255, 255, 127], i64::MAX as u64, 8)]
    #[case(vec![0, 0, 0, 0, 0, 0, 0, 0], f64::MIN as u64, 8)]
    #[case(vec![255, 255, 255, 255, 255, 255, 255, 255], f64::MAX as u64, 8)]
    fn test_fixed64_decoding(
        #[case] bytes: Vec<u8>,
        #[case] expected_value: u64,
        #[case] expected_len: usize,
    ) {
        // Act
        let (value, len) = decode_fixed64(&bytes).unwrap().unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(len, expected_len);
    }

    #[test]
    fn test_fixed64_decoding_empty_buffer() {
        // Arrange
        let bytes = Vec::new();

        // Act
        let result = decode_fixed64(&bytes).unwrap();

        // Assert
        assert_eq!(result, None);
    }
}
