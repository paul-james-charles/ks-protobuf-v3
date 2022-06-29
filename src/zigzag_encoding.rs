/// Encoded a 32 bit sign integer with zigzag encoding
///
/// ZigZag unlike varint encoding ensures that small negative numbers require less storage, unlike
/// Varint encoding which would use the a large amount of bytes because of the leading 1s.
///
/// Basic usage:
/// ```
/// use ks_protobuf_v3::encode_zigzag32;
///
/// let encoded_value = encode_zigzag32(-100);
/// ```
pub fn encode_zigzag32(value: i32) -> u32 {
    ((value << 1) ^ (value >> 31)) as u32
}

/// Decode a 32 bit zigzag encoded integer to a signed 32bit integer.
///
/// ZigZag unlike varint encoding ensures that small negative numbers require less storage, unlike
/// Varint encoding which would use the a large amount of bytes because of the leading 1s.
///
/// Basic usage:
/// ```
/// use ks_protobuf_v3::decode_zigzag32;
///
/// let decoded_value = decode_zigzag32(100);
/// ```
pub fn decode_zigzag32(value: u32) -> i32 {
    ((value >> 1) as i32) ^ (-((value & 1) as i32))
}

/// Encoded a 64 bit sign integer with zigzag encoding
///
/// ZigZag unlike varint encoding ensures that small negative numbers require less storage, unlike
/// Varint encoding which would use the a large amount of bytes because of the leading 1s.
///
/// Basic usage:
/// ```
/// use ks_protobuf_v3::encode_zigzag64;
///
/// let encoded_value = encode_zigzag64(-100);
/// ```
pub fn encode_zigzag64(value: i64) -> u64 {
    ((value << 1) ^ (value >> 31)) as u64
}

/// Decode a 64 bit zigzag encoded integer to a signed 32bit integer.
///
/// ZigZag unlike varint encoding ensures that small negative numbers require less storage, unlike
/// Varint encoding which would use the a large amount of bytes because of the leading 1s.
///
/// Basic usage:
/// ```
/// use ks_protobuf_v3::decode_zigzag64;
///
/// let decoded_value = decode_zigzag64(100);
/// ```
pub fn decode_zigzag64(value: u64) -> i64 {
    ((value >> 1) as i64) ^ (-((value & 1) as i64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(-10, 19)]
    #[case(-1, 1)]
    #[case(0, 0)]
    #[case(1, 2)]
    #[case(10, 20)]
    fn test_encode_zigzag32(#[case] value: i32, #[case] expected_result: u32) {
        // Act
        let result = encode_zigzag32(value);

        // Assert
        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(19, -10)]
    #[case(1, -1)]
    #[case(0, 0)]
    #[case(2, 1)]
    #[case(20, 10)]
    fn test_decode_zigzag32(#[case] value: u32, #[case] expected_result: i32) {
        // Act
        let result = decode_zigzag32(value);

        // Assert
        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(-10, 19)]
    #[case(-1, 1)]
    #[case(0, 0)]
    #[case(1, 2)]
    #[case(10, 20)]
    fn test_encode_zigzag64(#[case] value: i64, #[case] expected_result: u64) {
        // Act
        let result = encode_zigzag64(value);

        // Assert
        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(19, -10)]
    #[case(1, -1)]
    #[case(0, 0)]
    #[case(2, 1)]
    #[case(20, 10)]
    fn test_decode_zigzag64(#[case] value: u64, #[case] expected_result: i64) {
        // Act
        let result = decode_zigzag64(value);

        // Assert
        assert_eq!(result, expected_result);
    }
}
