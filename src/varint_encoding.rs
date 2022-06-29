use crate::{Buffer, DecodeError};

/// Encodes a 32 bit unsigned integer to LEB128
///
/// Varint encoding makes the assumption that most integer data types are used to store small
/// numbers and as such can be stored in a small number of bytes. This does have the issue of
/// storing large integers in more bytes. However it is a tradeoff.
///
/// Note: Storing of negative numbers is not recommended as they will require the full storage
/// length because 2's compliment will have leading 1s.
///
/// This varint32 encoding ensure 32bit unsigned values are stored between 0 and 5 bytes
///
/// Basic usage:
/// ```
/// use ks_protobuf_v3::{Buffer, encode_varint32};
///
/// let mut buffer = Buffer::default();
///
/// let encoded_length = encode_varint32(20, &mut buffer);
/// ```
pub fn encode_varint32(mut value: u32, buffer: &mut Buffer) -> usize {
    fn iter(value: &mut u32, buffer: &mut Buffer) -> bool {
        if (*value & !0x7F) > 0 {
            buffer.put_u8(((*value & 0x7F) | 0x80) as u8);
            *value >>= 7;
            true
        } else {
            buffer.put_u8(*value as u8);
            false
        }
    }

    for i in 1..=4 {
        if !iter(&mut value, buffer) {
            return i;
        }
    }

    buffer.put_u8(value as u8);
    5
}

/// Encodes a 64 bit unsigned integer to LEB128
///
/// Varint encoding makes the assumption that most integer data types are used to store small
/// numbers and as such can be stored in a small number of bytes. This does have the issue of
/// storing large integers in more bytes. However it is a tradeoff.
///
/// Note: Storing of negative numbers is not recommended as they will require the full storage
/// length because 2's compliment will have leading 1s.
///
/// This varint64 encoding ensure 64bit unsigned values are stored between 0 and 5 bytes
///
/// Basic usage:
/// ```
/// use ks_protobuf_v3::{Buffer, encode_varint64};
///
/// let mut buffer = Buffer::default();
///
/// let encoded_length = encode_varint64(20, &mut buffer);
/// ```
pub fn encode_varint64(mut value: u64, buffer: &mut Buffer) -> usize {
    fn iter(value: &mut u64, buffer: &mut Buffer) -> bool {
        if (*value & !0x7F) > 0 {
            buffer.put_u8(((*value & 0x7F) | 0x80) as u8);
            *value >>= 7;
            true
        } else {
            buffer.put_u8(*value as u8);
            false
        }
    }

    for i in 1..=9 {
        if !iter(&mut value, buffer) {
            return i;
        }
    }

    buffer.put_u8(value as u8);
    10
}

/// Decodes a Varint to a 32 bit unsigned integer
///
/// Basic usage:
/// ```
/// use ks_protobuf_v3::{Buffer, decode_varint32};
///
/// let mut buffer: Vec<u8> = vec![0];
///
/// let (value, len) = decode_varint32(&buffer).unwrap().unwrap();
/// ```
pub fn decode_varint32(data: &[u8]) -> Result<Option<(u32, usize)>, DecodeError> {
    decode_varint(data)
}

/// Decodes a Varint to a 64 bit unsigned integer
///
/// Basic usage:
/// ```
/// use ks_protobuf_v3::{Buffer, decode_varint64};
///
/// let mut buffer: Vec<u8> = vec![0];
///
/// let (value, len) = decode_varint64(&buffer).unwrap().unwrap();
/// ```
pub fn decode_varint64(data: &[u8]) -> Result<Option<(u64, usize)>, DecodeError> {
    decode_varint(data)
}

// pub fn get_varint_encoded_length(buffer: &[u8], start_position: usize) -> usize {
//     1
// }

const MAX_VARINT_ENCODED_LEN: usize = 10;
const MAX_VARINT32_ENCODED_LEN: usize = 5;

trait DecodeVarint {
    const MAX_ENCODED_LEN: usize;
    const LAST_BYTE_MAXVALUE: u8;

    fn from_u64(value: u64) -> Self;
}

impl DecodeVarint for u64 {
    const MAX_ENCODED_LEN: usize = MAX_VARINT_ENCODED_LEN;
    const LAST_BYTE_MAXVALUE: u8 = 0x01;

    fn from_u64(value: u64) -> Self {
        value
    }
}

impl DecodeVarint for u32 {
    const MAX_ENCODED_LEN: usize = MAX_VARINT32_ENCODED_LEN;
    const LAST_BYTE_MAXVALUE: u8 = 0x0F;

    fn from_u64(value: u64) -> Self {
        value as u32
    }
}

fn decode_varint<D: DecodeVarint>(data: &[u8]) -> Result<Option<(D, usize)>, DecodeError> {
    if !data.is_empty() && data[0] < 0x80 {
        let ret = data[0] as u64;
        let consume = 1;
        Ok(Some((D::from_u64(ret), consume)))
    } else if data.len() >= 2 && data[1] < 0x80 {
        let ret = (data[0] & 0x7f) as u64 | (data[1] as u64) << 7;
        let consume = 2;
        Ok(Some((D::from_u64(ret), consume)))
    } else {
        decode_varint_slow(data)
    }
}

fn decode_varint_slow<D: DecodeVarint>(data: &[u8]) -> Result<Option<(D, usize)>, DecodeError> {
    let mut r: u64 = 0;
    for (i, &b) in data.iter().enumerate() {
        if i == D::MAX_ENCODED_LEN - 1 {
            if b > D::LAST_BYTE_MAXVALUE {
                return Err(DecodeError::BufferOverrun);
            }
            let r = r | ((b as u64) << (i as u64 * 7));

            return Ok(Some((D::from_u64(r), i + 1)));
        }

        r |= ((b & 0x7f) as u64) << (i as u64 * 7);
        if b < 0x80 {
            return Ok(Some((D::from_u64(r), i + 1)));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Buffer;
    use rstest::*;

    #[rstest]
    #[case(1, vec![1])]
    #[case(u8::MIN as u32, vec![0])]
    #[case(u8::MAX as u32, vec![255, 1])]
    #[case(i8::MIN as u32, vec![128, 255, 255, 255, 15])]
    #[case(i8::MAX as u32, vec![127])]
    #[case(u16::MAX as u32, vec![255, 255, 3])]
    #[case(i16::MIN as u32, vec![128, 128, 254, 255, 15])]
    #[case(i16::MAX as u32, vec![255, 255, 1])]
    #[case(u32::MAX, vec![255, 255, 255, 255, 15])]
    #[case(i32::MIN as u32, vec![128, 128, 128, 128, 8])]
    #[case(i32::MAX as u32, vec![255, 255, 255, 255, 7])]
    fn test_varint32_encoding(#[case] value: u32, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = encode_varint32(value, &mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(1, vec![1])]
    #[case(u8::MIN as u64, vec![0])]
    #[case(u8::MAX as u64, vec![255, 1])]
    #[case(i8::MIN as u64, vec![128, 255, 255, 255, 255, 255, 255, 255, 255, 1])]
    #[case(i8::MAX as u64, vec![127])]
    #[case(u16::MAX as u64, vec![255, 255, 3])]
    #[case(i16::MIN as u64, vec![128, 128, 254, 255, 255, 255, 255, 255, 255, 1])]
    #[case(i16::MAX as u64, vec![255, 255, 1])]
    #[case(u32::MAX as u64, vec![255, 255, 255, 255, 15])]
    #[case(i32::MIN as u64, vec![128, 128, 128, 128, 248, 255, 255, 255, 255, 1])]
    #[case(i32::MAX as u64, vec![255, 255, 255, 255, 7])]
    #[case(u64::MAX, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1])]
    #[case(i64::MIN as u64, vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 1])]
    #[case(i64::MAX as u64, vec![255, 255, 255, 255, 255, 255, 255, 255, 127])]
    fn test_varint64_encoding(#[case] value: u64, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = encode_varint64(value, &mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![1], 1, 1)]
    #[case(vec![0], u8::MIN as u32, 1)]
    #[case(vec![255, 1], u8::MAX as u32, 2)]
    #[case(vec![128, 255, 255, 255, 15], i8::MIN as u32, 5)]
    #[case(vec![127], i8::MAX as u32, 1)]
    #[case(vec![255, 255, 3], u16::MAX as u32, 3)]
    #[case(vec![128, 128, 254, 255, 15], i16::MIN as u32, 5)]
    #[case(vec![255, 255, 1], i16::MAX as u32, 3)]
    #[case(vec![255, 255, 255, 255, 15], u32::MAX, 5)]
    #[case(vec![128, 128, 128, 128, 8], i32::MIN as u32, 5)]
    #[case(vec![255, 255, 255, 255, 7], i32::MAX as u32, 5)]
    fn test_varint32_decoding(
        #[case] bytes: Vec<u8>,
        #[case] expected_value: u32,
        #[case] expected_len: usize,
    ) {
        // Act
        let (value, len) = decode_varint32(&bytes).unwrap().unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(len, expected_len);
    }

    #[rstest]
    #[case(vec![1], 1, 1)]
    #[case(vec![0], u8::MIN as u64, 1)]
    #[case(vec![255, 1], u8::MAX as u64, 2)]
    #[case(vec![128, 255, 255, 255, 255, 255, 255, 255, 255, 1], i8::MIN as u64, 10)]
    #[case(vec![127], i8::MAX as u64, 1)]
    #[case(vec![255, 255, 3], u16::MAX as u64, 3)]
    #[case(vec![128, 128, 254, 255, 255, 255, 255, 255, 255, 1], i16::MIN as u64, 10)]
    #[case(vec![255, 255, 1], i16::MAX as u64, 3)]
    #[case(vec![255, 255, 255, 255, 15], u32::MAX as u64, 5)]
    #[case(vec![128, 128, 128, 128, 248, 255, 255, 255, 255, 1], i32::MIN as u64, 10)]
    #[case(vec![255, 255, 255, 255, 7], i32::MAX as u64, 5)]
    #[case(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1], u64::MAX, 10)]
    #[case(vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 1], i64::MIN as u64, 10)]
    #[case(vec![255, 255, 255, 255, 255, 255, 255, 255, 127], i64::MAX as u64, 9)]
    fn test_varint64_decoding(
        #[case] bytes: Vec<u8>,
        #[case] expected_value: u64,
        #[case] expected_len: usize,
    ) {
        // Act
        let (value, len) = decode_varint64(&bytes).unwrap().unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(len, expected_len);
    }

    #[test]
    fn test_varint32_decoding_empty_buffer() {
        // Arrange
        let bytes = Vec::new();

        // Act
        let result = decode_varint32(&bytes).unwrap();

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn test_varint64_decoding_empty_buffer() {
        // Arrange
        let bytes = Vec::new();

        // Act
        let result = decode_varint64(&bytes).unwrap();

        // Assert
        assert_eq!(result, None);
    }
}
