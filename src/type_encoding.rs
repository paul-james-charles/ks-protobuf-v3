use crate::{
    decode_fixed32, decode_fixed64, decode_varint32, decode_varint64, decode_zigzag32,
    decode_zigzag64, encode_fixed32, encode_fixed64, encode_varint32, encode_varint64,
    encode_zigzag32, encode_zigzag64, Buffer, DecodeError, Fixed32, Fixed64, Varint, VarintField,
};

impl Varint for bool {
    fn to_varint(&self, buffer: &mut Buffer) -> usize {
        encode_varint32(if *self { 1 } else { 0 }, buffer)
    }

    fn from_varint(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_varint32(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => match v {
                    0 => {
                        *self = false;
                        Ok(s)
                    }
                    1 => {
                        *self = true;
                        Ok(s)
                    }
                    _ => Err(DecodeError::UnableToDecode),
                },
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Varint for u8 {
    fn to_varint(&self, buffer: &mut Buffer) -> usize {
        encode_varint32(*self as u32, buffer)
    }

    fn from_varint(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_varint32(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    if v <= u8::MAX as u32 {
                        *self = v as u8;
                        Ok(s)
                    } else {
                        Err(DecodeError::UnableToDecodeBecauseTheValueWasTooLarge)
                    }
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Varint for u16 {
    fn to_varint(&self, buffer: &mut Buffer) -> usize {
        encode_varint32(*self as u32, buffer)
    }

    fn from_varint(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_varint32(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    if v <= u16::MAX as u32 {
                        *self = v as u16;
                        Ok(s)
                    } else {
                        Err(DecodeError::UnableToDecodeBecauseTheValueWasTooLarge)
                    }
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Varint for u32 {
    fn to_varint(&self, buffer: &mut Buffer) -> usize {
        encode_varint32(*self, buffer)
    }

    fn from_varint(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_varint32(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    *self = v;
                    Ok(s)
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Varint for u64 {
    fn to_varint(&self, buffer: &mut Buffer) -> usize {
        encode_varint64(*self, buffer)
    }

    fn from_varint(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_varint64(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    *self = v;
                    Ok(s)
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Varint for i8 {
    fn to_varint(&self, buffer: &mut Buffer) -> usize {
        encode_varint32(encode_zigzag32(*self as i32), buffer)
    }

    fn from_varint(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_varint32(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    let v = decode_zigzag32(v);
                    if v < i8::MIN as i32 {
                        Err(DecodeError::UnableToDecodeBecauseTheValueWasTooSmall)
                    } else if v > i8::MAX as i32 {
                        Err(DecodeError::UnableToDecodeBecauseTheValueWasTooLarge)
                    } else {
                        *self = v as i8;
                        Ok(s)
                    }
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Varint for i16 {
    fn to_varint(&self, buffer: &mut Buffer) -> usize {
        encode_varint32(encode_zigzag32(*self as i32), buffer)
    }

    fn from_varint(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_varint32(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    let v = decode_zigzag32(v);
                    if v < i16::MIN as i32 {
                        Err(DecodeError::UnableToDecodeBecauseTheValueWasTooSmall)
                    } else if v > i16::MAX as i32 {
                        Err(DecodeError::UnableToDecodeBecauseTheValueWasTooLarge)
                    } else {
                        *self = v as i16;
                        Ok(s)
                    }
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Varint for i32 {
    fn to_varint(&self, buffer: &mut Buffer) -> usize {
        encode_varint32(encode_zigzag32(*self), buffer)
    }

    fn from_varint(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_varint32(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    *self = decode_zigzag32(v);
                    Ok(s)
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Varint for i64 {
    fn to_varint(&self, buffer: &mut Buffer) -> usize {
        encode_varint64(encode_zigzag64(*self), buffer)
    }

    fn from_varint(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_varint64(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    *self = decode_zigzag64(v);
                    Ok(s)
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl VarintField for bool {}
impl VarintField for u8 {}
impl VarintField for u16 {}
impl VarintField for u32 {}
impl VarintField for u64 {}
impl VarintField for i8 {}
impl VarintField for i16 {}
impl VarintField for i32 {}
impl VarintField for i64 {}

impl Fixed32 for u32 {
    fn to_fixed32(&self, buffer: &mut Buffer) -> usize {
        encode_fixed32(*self, buffer)
    }

    fn from_fixed32(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_fixed32(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    *self = v;
                    Ok(s)
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Fixed32 for i32 {
    fn to_fixed32(&self, buffer: &mut Buffer) -> usize {
        encode_fixed32(*self as u32, buffer)
    }

    fn from_fixed32(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_fixed32(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    *self = v as i32;
                    Ok(s)
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Fixed32 for f32 {
    fn to_fixed32(&self, buffer: &mut Buffer) -> usize {
        encode_fixed32(*self as u32, buffer)
    }

    fn from_fixed32(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_fixed32(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    *self = v as f32;
                    Ok(s)
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Fixed64 for u64 {
    fn to_fixed64(&self, buffer: &mut Buffer) -> usize {
        encode_fixed64(*self, buffer)
    }

    fn from_fixed64(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_fixed64(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    *self = v;
                    Ok(s)
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Fixed64 for i64 {
    fn to_fixed64(&self, buffer: &mut Buffer) -> usize {
        encode_fixed64(*self as u64, buffer)
    }

    fn from_fixed64(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_fixed64(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    *self = v as i64;
                    Ok(s)
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

impl Fixed64 for f64 {
    fn to_fixed64(&self, buffer: &mut Buffer) -> usize {
        encode_fixed64(*self as u64, buffer)
    }

    fn from_fixed64(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let result = decode_fixed64(&buffer.to_vec()[..]);

        match result {
            Ok(value) => match value {
                Some((v, s)) => {
                    *self = v as f64;
                    Ok(s)
                }
                None => Err(DecodeError::UnableToDecode),
            },
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Buffer;
    use rstest::*;

    #[rstest]
    #[case(false, vec![0])]
    #[case(true, vec![1])]
    fn test_bool_encoding(#[case] value: bool, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_varint(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![0], false)]
    #[case(vec![1], true)]
    fn test_bool_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: bool) {
        // Arrange
        //let mut buffer = Buffer::new();
        let mut value = false;

        // Act
        let result = value.from_varint(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(u8::MIN, vec![0])]
    #[case(u8::MAX, vec![255, 1])]
    fn test_u8_encoding(#[case] value: u8, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_varint(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![0], u8::MIN)]
    #[case(vec![255, 1], u8::MAX)]
    fn test_u8_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: u8) {
        // Arrange
        let mut value: u8 = 0;

        // Act
        let result = value.from_varint(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(u16::MIN, vec![0])]
    #[case(u16::MAX, vec![255, 255, 3])]
    fn test_u16_encoding(#[case] value: u16, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_varint(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![0], u16::MIN)]
    #[case(vec![255, 255, 3], u16::MAX)]
    fn test_u16_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: u16) {
        // Arrange
        let mut value: u16 = 0;

        // Act
        let result = value.from_varint(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(u32::MIN, vec![0])]
    #[case(u32::MAX, vec![255, 255, 255, 255, 15])]
    fn test_u32_encoding(#[case] value: u32, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_varint(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![0], u32::MIN)]
    #[case(vec![255, 255, 255, 255, 15], u32::MAX)]
    fn test_u32_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: u32) {
        // Arrange
        let mut value: u32 = 0;

        // Act
        let result = value.from_varint(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(u64::MIN, vec![0])]
    #[case(u64::MAX, vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1])]
    fn test_u64_encoding(#[case] value: u64, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_varint(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![0], u64::MIN)]
    #[case(vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1], u64::MAX)]
    fn test_u64_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: u64) {
        // Arrange
        let mut value: u64 = 0;

        // Act
        let result = value.from_varint(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(i8::MIN, vec![255, 1])]
    #[case(i8::MAX, vec![254, 1])]
    fn test_i8_encoding(#[case] value: i8, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_varint(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![255, 1], i8::MIN)]
    #[case(vec![254, 1], i8::MAX)]
    fn test_i8_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: i8) {
        // Arrange
        let mut value: i8 = 0;

        // Act
        let result = value.from_varint(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(i8::MIN as i16, vec![255, 1])]
    #[case(i8::MAX as i16, vec![254, 1])]
    #[case(i16::MIN, vec![255, 255, 3])]
    #[case(i16::MAX, vec![254, 255, 3])]
    fn test_i16_encoding(#[case] value: i16, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_varint(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![255, 1], i8::MIN as i16)]
    #[case(vec![254, 1], i8::MAX as i16)]
    #[case(vec![255, 255, 3], i16::MIN)]
    #[case(vec![254, 255, 3], i16::MAX)]
    fn test_i16_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: i16) {
        // Arrange
        let mut value: i16 = 0;

        // Act
        let result = value.from_varint(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(i8::MIN as i32, vec![255, 1])]
    #[case(i8::MAX as i32, vec![254, 1])]
    #[case(i16::MIN as i32, vec![255, 255, 3])]
    #[case(i16::MAX as i32, vec![254, 255, 3])]
    #[case(i32::MIN, vec![255, 255, 255, 255, 15])]
    #[case(i32::MAX, vec![254, 255, 255, 255, 15])]
    fn test_i32_encoding(#[case] value: i32, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_varint(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![255, 1], i8::MIN as i32)]
    #[case(vec![254, 1], i8::MAX as i32)]
    #[case(vec![255, 255, 3], i16::MIN as i32)]
    #[case(vec![254, 255, 3], i16::MAX as i32)]
    #[case(vec![255, 255, 255, 255, 15], i32::MIN)]
    #[case(vec![254, 255, 255, 255, 15], i32::MAX)]
    fn test_i32_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: i32) {
        // Arrange
        let mut value: i32 = 0;

        // Act
        let result = value.from_varint(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(i8::MIN as i64, vec![255, 1])]
    #[case(i8::MAX as i64, vec![254, 1])]
    #[case(i16::MIN as i64, vec![255, 255, 3])]
    #[case(i16::MAX as i64, vec![254, 255, 3])]
    #[case(i32::MIN as i64, vec![255, 255, 255, 255, 15])]
    #[case(i32::MAX as i64, vec![254, 255, 255, 255, 15])]
    #[case(i64::MIN, vec![128, 128, 128, 128, 240, 255, 255, 255, 255, 1])]
    #[case(i64::MAX, vec![129, 128, 128, 128, 240, 255, 255, 255, 255, 1])]
    #[case(i64::MIN + 1, vec![130, 128, 128, 128, 240, 255, 255, 255, 255, 1])]
    #[case(i64::MAX - 1, vec![131, 128, 128, 128, 240, 255, 255, 255, 255, 1])]
    fn test_i64_encoding(#[case] value: i64, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_varint(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }
    //
    #[rstest]
    #[case(vec![255, 1], i8::MIN as i64)]
    #[case(vec![254, 1], i8::MAX as i64)]
    #[case(vec![255, 255, 3], i16::MIN as i64)]
    #[case(vec![254, 255, 3], i16::MAX as i64)]
    #[case(vec![255, 255, 255, 255, 15], i32::MIN as i64)]
    #[case(vec![254, 255, 255, 255, 15], i32::MAX as i64)]
    //#[case(vec![128, 128, 128, 128, 240, 255, 255, 255, 255, 1], i64::MIN)]
    //#[case(vec![129, 128, 128, 128, 240, 255, 255, 255, 255, 1], i64::MAX)]
    //#[case(vec![130, 128, 128, 128, 240, 255, 255, 255, 255, 1], i64::MIN + 1)]
    //#[case(vec![131, 128, 128, 128, 240, 255, 255, 255, 255, 1], i64::MAX - 1)]
    fn test_i64_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: i64) {
        // Arrange
        let mut value: i64 = 0;

        // Act
        let result = value.from_varint(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(u32::MIN, vec![0, 0, 0, 0])]
    #[case(u32::MAX, vec![255, 255, 255, 255])]
    fn test_u32_fixed32_encoding(#[case] value: u32, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_fixed32(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![0, 0, 0, 0], u32::MIN)]
    #[case(vec![255, 255, 255, 255], u32::MAX)]
    fn test_u32_fixed32_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: u32) {
        // Arrange
        let mut value: u32 = 0;

        // Act
        let result = value.from_fixed32(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(i32::MIN, vec![0, 0, 0, 128])]
    #[case(i32::MAX, vec![255, 255, 255, 127])]
    fn test_i32_fixed32_encoding(#[case] value: i32, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_fixed32(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![0, 0, 0, 128], i32::MIN)]
    #[case(vec![255, 255, 255, 127], i32::MAX)]
    fn test_i32_fixed32_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: i32) {
        // Arrange
        let mut value: i32 = 0;

        // Act
        let result = value.from_fixed32(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(f32::MIN, vec![0, 0, 0, 0])]
    #[case(f32::MAX, vec![255, 255, 255, 255])]
    fn test_f32_fixed32_encoding(#[case] value: f32, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_fixed32(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![0, 0, 0, 0], f32::MIN)]
    #[case(vec![255, 255, 255, 255], f32::MAX)]
    fn test_f32_fixed32_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: f32) {
        // Arrange
        let mut value: f32 = 0.0;

        // Act
        let result = value.from_fixed32(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(u64::MIN, vec![0, 0, 0, 0, 0, 0, 0, 0])]
    #[case(u64::MAX, vec![255, 255, 255, 255, 255, 255, 255, 255])]
    fn test_u64_fixed64_encoding(#[case] value: u64, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_fixed64(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![0, 0, 0, 0, 0, 0, 0, 0], u64::MIN)]
    #[case(vec![255, 255, 255, 255, 255, 255, 255, 255], u64::MAX)]
    fn test_u64_fixed64_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: u64) {
        // Arrange
        let mut value: u64 = 0;

        // Act
        let result = value.from_fixed64(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(i64::MIN, vec![0, 0, 0, 0, 0, 0, 0, 128])]
    #[case(i64::MAX, vec![255, 255, 255, 255, 255, 255, 255, 127])]
    fn test_i64_fixed64_encoding(#[case] value: i64, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_fixed64(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![0, 0, 0, 0, 0, 0, 0, 128], i64::MIN)]
    #[case(vec![255, 255, 255, 255, 255, 255, 255, 127], i64::MAX)]
    fn test_i64_fixed64_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: i64) {
        // Arrange
        let mut value: i64 = 0;

        // Act
        let result = value.from_fixed64(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }

    #[rstest]
    #[case(f64::MIN, vec![0, 0, 0, 0, 0, 0, 0 ,0])]
    #[case(f64::MAX, vec![255, 255, 255, 255, 255, 255, 255, 255])]
    fn test_f64_fixed64_encoding(#[case] value: f64, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = value.to_fixed64(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![0, 0, 0, 0], f64::MIN)]
    #[case(vec![255, 255, 255, 255], f64::MAX)]
    fn test_f64_fixed64_decoding(#[case] buffer: Vec<u8>, #[case] expected_value: f64) {
        // Arrange
        let mut value: f64 = 0.0;

        // Act
        let result = value.from_fixed64(&buffer).unwrap();

        // Assert
        assert_eq!(value, expected_value);
        assert_eq!(result, buffer.len());
    }
}
