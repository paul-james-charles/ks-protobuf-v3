#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct DataBuffer {
        data: Vec<u8>,
    }

    impl BufferTrait for DataBuffer {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn add_byte_to_end(&mut self, value: u8) {
            self.data.push(value)
        }

        fn add_bytes_to_end(&mut self, value: &[u8]) {
            for byte in value {
                self.data.push(*byte);
            }
        }

        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    #[test]
    fn test_encode_int_as_varint32() {
        // Arrange
        struct Message {
            i8_min: i8,
            i8_max: i8,
            i16_min: i16,
            i16_max: i16,
            i32_min: i32,
            i32_max: i32,
            u8_min: u8,
            u8_max: u8,
            u16_min: u16,
            u16_max: u16,
            u32_min: u32,
            u32_max: u32,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();
                Self::encode_message_int_as_varint32(&mut buffer, 1, self.i8_min as i32);
                Self::encode_message_int_as_varint32(&mut buffer, 2, self.i8_max as i32);
                Self::encode_message_int_as_varint32(&mut buffer, 3, self.i16_min as i32);
                Self::encode_message_int_as_varint32(&mut buffer, 4, self.i16_max as i32);
                Self::encode_message_int_as_varint32(&mut buffer, 5, self.i32_min as i32);
                Self::encode_message_int_as_varint32(&mut buffer, 6, self.i32_max as i32);

                Self::encode_message_int_as_varint32(&mut buffer, 7, self.u8_min as i32);
                Self::encode_message_int_as_varint32(&mut buffer, 8, self.u8_max as i32);
                Self::encode_message_int_as_varint32(&mut buffer, 9, self.u16_min as i32);
                Self::encode_message_int_as_varint32(&mut buffer, 10, self.u16_max as i32);
                Self::encode_message_int_as_varint32(&mut buffer, 11, self.u32_min as i32);
                Self::encode_message_int_as_varint32(&mut buffer, 12, self.u32_max as i32);

                Ok(buffer)
            }
        }

        let message = Message {
            i8_min: i8::MIN,
            i8_max: i8::MAX,
            i16_min: i16::MIN,
            i16_max: i16::MAX,
            i32_min: i32::MIN,
            i32_max: i32::MAX,
            u8_min: u8::MIN,
            u8_max: u8::MAX,
            u16_min: u16::MIN,
            u16_max: u16::MAX,
            u32_min: u32::MIN,
            u32_max: u32::MAX,
        };

        let expected_result: Vec<u8> = vec![
            8, 128, 255, 255, 255, 15, // i8_min
            16, 127, // i8_max
            24, 128, 128, 254, 255, 15, // i16_min
            32, 255, 255, 1, // i16_max
            40, 128, 128, 128, 128, 8, // i32_min
            48, 255, 255, 255, 255, 7, // i32_max
            56, 0, // u8_min
            64, 255, 1, // u8_max
            72, 0, // u16_min
            80, 255, 255, 3, // u16_max
            88, 0, // u32_min
            96, 255, 255, 255, 255, 15, // u32_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_uint_as_varint32() {
        // Arrange
        struct Message {
            u8_min: u8,
            u8_max: u8,
            u16_min: u16,
            u16_max: u16,
            u32_min: u32,
            u32_max: u32,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_uint_as_varint32(&mut buffer, 1, self.u8_min as u32);
                Self::encode_message_uint_as_varint32(&mut buffer, 2, self.u8_max as u32);
                Self::encode_message_uint_as_varint32(&mut buffer, 3, self.u16_min as u32);
                Self::encode_message_uint_as_varint32(&mut buffer, 4, self.u16_max as u32);
                Self::encode_message_uint_as_varint32(&mut buffer, 5, self.u32_min);
                Self::encode_message_uint_as_varint32(&mut buffer, 6, self.u32_max);

                Ok(buffer)
            }
        }

        let message = Message {
            u8_min: u8::MIN,
            u8_max: u8::MAX,
            u16_min: u16::MIN,
            u16_max: u16::MAX,
            u32_min: u32::MIN,
            u32_max: u32::MAX,
        };

        let expected_result: Vec<u8> = vec![
            8, 0, // u8_min
            16, 255, 1, // u8_max
            24, 0, // u16_min
            32, 255, 255, 3, // u16_max
            40, 0, // u32_min
            48, 255, 255, 255, 255, 15, // u32_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_int_as_varint64() {
        // Arrange
        struct Message {
            i8_min: i8,
            i8_max: i8,
            i16_min: i16,
            i16_max: i16,
            i32_min: i32,
            i32_max: i32,
            i64_min: i64,
            i64_max: i64,
            u8_min: u8,
            u8_max: u8,
            u16_min: u16,
            u16_max: u16,
            u32_min: u32,
            u32_max: u32,
            u64_min: u64,
            u64_max: u64,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();
                Self::encode_message_int_as_varint64(&mut buffer, 1, self.i8_min as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 2, self.i8_max as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 3, self.i16_min as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 4, self.i16_max as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 5, self.i32_min as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 6, self.i32_max as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 7, self.i64_min as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 8, self.i64_max as i64);

                Self::encode_message_int_as_varint64(&mut buffer, 9, self.u8_min as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 10, self.u8_max as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 11, self.u16_min as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 12, self.u16_max as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 13, self.u32_min as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 14, self.u32_max as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 15, self.u64_min as i64);
                Self::encode_message_int_as_varint64(&mut buffer, 16, self.u64_max as i64);

                Ok(buffer)
            }
        }

        let message = Message {
            i8_min: i8::MIN,
            i8_max: i8::MAX,
            i16_min: i16::MIN,
            i16_max: i16::MAX,
            i32_min: i32::MIN,
            i32_max: i32::MAX,
            i64_min: i64::MIN,
            i64_max: i64::MAX,
            u8_min: u8::MIN,
            u8_max: u8::MAX,
            u16_min: u16::MIN,
            u16_max: u16::MAX,
            u32_min: u32::MIN,
            u32_max: u32::MAX,
            u64_min: u64::MIN,
            u64_max: u64::MAX,
        };

        let expected_result: Vec<u8> = vec![
            8, 128, 255, 255, 255, 255, 255, 255, 255, 255, 1, // i8_min
            16, 127, // i8_max
            24, 128, 128, 254, 255, 255, 255, 255, 255, 255, 1, // i16_min
            32, 255, 255, 1, // i16_max
            40, 128, 128, 128, 128, 248, 255, 255, 255, 255, 1, // i32_min
            48, 255, 255, 255, 255, 7, // i32_max
            56, 128, 128, 128, 128, 128, 128, 128, 128, 128, 1, // i64_min
            64, 255, 255, 255, 255, 255, 255, 255, 255, 127, // i64_max
            72, 0, // u8_min
            80, 255, 1, // u8_max
            88, 0, // u16_min
            96, 255, 255, 3, // u16_max
            104, 0, // u32_min
            112, 255, 255, 255, 255, 15, // u32_max
            120, 0, // u64_min
            128, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, // u64_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_uint_as_varint64() {
        // Arrange
        struct Message {
            u8_min: u8,
            u8_max: u8,
            u16_min: u16,
            u16_max: u16,
            u32_min: u32,
            u32_max: u32,
            u64_min: u64,
            u64_max: u64,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_uint_as_varint64(&mut buffer, 1, self.u8_min as u64);
                Self::encode_message_uint_as_varint64(&mut buffer, 2, self.u8_max as u64);
                Self::encode_message_uint_as_varint64(&mut buffer, 3, self.u16_min as u64);
                Self::encode_message_uint_as_varint64(&mut buffer, 4, self.u16_max as u64);
                Self::encode_message_uint_as_varint64(&mut buffer, 5, self.u32_min as u64);
                Self::encode_message_uint_as_varint64(&mut buffer, 6, self.u32_max as u64);
                Self::encode_message_uint_as_varint64(&mut buffer, 7, self.u64_min as u64);
                Self::encode_message_uint_as_varint64(&mut buffer, 8, self.u64_max as u64);

                Ok(buffer)
            }
        }

        let message = Message {
            u8_min: u8::MIN,
            u8_max: u8::MAX,
            u16_min: u16::MIN,
            u16_max: u16::MAX,
            u32_min: u32::MIN,
            u32_max: u32::MAX,
            u64_min: u64::MIN,
            u64_max: u64::MAX,
        };

        let expected_result: Vec<u8> = vec![
            8, 0, // u8_min
            16, 255, 1, // u8_max
            24, 0, // u16_min
            32, 255, 255, 3, // u16_max
            40, 0, // u32_min
            48, 255, 255, 255, 255, 15, // u32_max
            56, 0, // u64_min
            64, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, // u64_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_sint_as_varint32_with_zigzag_encoding() {
        // Arrange
        struct Message {
            s8_min: i8,
            s8_max: i8,
            s16_min: i16,
            s16_max: i16,
            s32_min: i32,
            s32_max: i32,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_sint_as_varint32_with_zigzag_encoding(
                    &mut buffer,
                    1,
                    self.s8_min as i32,
                );
                Self::encode_message_sint_as_varint32_with_zigzag_encoding(
                    &mut buffer,
                    2,
                    self.s8_max as i32,
                );
                Self::encode_message_sint_as_varint32_with_zigzag_encoding(
                    &mut buffer,
                    3,
                    self.s16_min as i32,
                );
                Self::encode_message_sint_as_varint32_with_zigzag_encoding(
                    &mut buffer,
                    4,
                    self.s16_max as i32,
                );
                Self::encode_message_sint_as_varint32_with_zigzag_encoding(
                    &mut buffer,
                    5,
                    self.s32_min,
                );
                Self::encode_message_sint_as_varint32_with_zigzag_encoding(
                    &mut buffer,
                    6,
                    self.s32_max,
                );

                Ok(buffer)
            }
        }

        let message = Message {
            s8_min: i8::MIN,
            s8_max: i8::MAX,
            s16_min: i16::MIN,
            s16_max: i16::MAX,
            s32_min: i32::MIN,
            s32_max: i32::MAX,
        };

        let expected_result: Vec<u8> = vec![
            8, 255, 1, // s8_min
            16, 254, 1, // s8_max
            24, 255, 255, 3, // s16_min
            32, 254, 255, 3, // s16_max
            40, 255, 255, 255, 255, 15, // s32_min
            48, 254, 255, 255, 255, 15, // s32_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_sint_as_varint64_with_zigzag_encoding() {
        // Arrange
        struct Message {
            s8_min: i8,
            s8_max: i8,
            s16_min: i16,
            s16_max: i16,
            s32_min: i32,
            s32_max: i32,
            s64_min: i64,
            s64_max: i64,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_sint_as_varint64_with_zigzag_encoding(
                    &mut buffer,
                    1,
                    self.s8_min as i64,
                );
                Self::encode_message_sint_as_varint64_with_zigzag_encoding(
                    &mut buffer,
                    2,
                    self.s8_max as i64,
                );
                Self::encode_message_sint_as_varint64_with_zigzag_encoding(
                    &mut buffer,
                    3,
                    self.s16_min as i64,
                );
                Self::encode_message_sint_as_varint64_with_zigzag_encoding(
                    &mut buffer,
                    4,
                    self.s16_max as i64,
                );
                Self::encode_message_sint_as_varint64_with_zigzag_encoding(
                    &mut buffer,
                    5,
                    self.s32_min as i64,
                );
                Self::encode_message_sint_as_varint64_with_zigzag_encoding(
                    &mut buffer,
                    6,
                    self.s32_max as i64,
                );
                Self::encode_message_sint_as_varint64_with_zigzag_encoding(
                    &mut buffer,
                    7,
                    self.s64_min,
                );
                Self::encode_message_sint_as_varint64_with_zigzag_encoding(
                    &mut buffer,
                    8,
                    self.s64_max,
                );

                Ok(buffer)
            }
        }

        let message = Message {
            s8_min: i8::MIN,
            s8_max: i8::MAX,
            s16_min: i16::MIN,
            s16_max: i16::MAX,
            s32_min: i32::MIN,
            s32_max: i32::MAX,
            s64_min: i64::MIN,
            s64_max: i64::MAX,
        };

        let expected_result: Vec<u8> = vec![
            8, 255, 1, // s8_min
            16, 254, 1, // s8_max
            24, 255, 255, 3, // s16_min
            32, 254, 255, 3, // s16_max
            40, 255, 255, 255, 255, 15, // s32_min
            48, 254, 255, 255, 255, 15, // s32_max
            56, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, // s64_min
            64, 254, 255, 255, 255, 255, 255, 255, 255, 255, 1, // s64_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_misc_as_varint32() {
        // Arrange
        struct Message {
            bool_true: bool,
            bool_false: bool,
            enum_value_1: u32,
            enum_value_2: u32,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_bool_as_varint32(&mut buffer, 1, self.bool_true);
                Self::encode_message_bool_as_varint32(&mut buffer, 2, self.bool_false);
                Self::encode_message_enum_as_varint32(&mut buffer, 3, self.enum_value_1);
                Self::encode_message_enum_as_varint32(&mut buffer, 4, self.enum_value_2);

                Ok(buffer)
            }
        }

        let message = Message {
            bool_true: true,
            bool_false: false,
            enum_value_1: 1,
            enum_value_2: 2,
        };

        let expected_result: Vec<u8> = vec![
            8, 1, // bool_true
            16, 0, // bool_false
            24, 1, // enum_value_1
            32, 2, // enum_value_2
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_uint_as_fixed32() {
        // Arrange
        struct Message {
            u32_min: u32,
            u32_max: u32,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_uint_as_fixed32(&mut buffer, 1, self.u32_min);
                Self::encode_message_uint_as_fixed32(&mut buffer, 2, self.u32_max);

                Ok(buffer)
            }
        }

        let message = Message {
            u32_min: u32::MIN,
            u32_max: u32::MAX,
        };

        let expected_result: Vec<u8> = vec![
            13, 0, 0, 0, 0, // u32_min
            21, 255, 255, 255, 255, // u32_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_sint_as_sfixed32() {
        // Arrange
        struct Message {
            i32_min: i32,
            i32_max: i32,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_sint_as_sfixed32(&mut buffer, 1, self.i32_min);
                Self::encode_message_sint_as_sfixed32(&mut buffer, 2, self.i32_max);

                Ok(buffer)
            }
        }

        let message = Message {
            i32_min: i32::MIN,
            i32_max: i32::MAX,
        };

        let expected_result: Vec<u8> = vec![
            13, 0, 0, 0, 128, // i32_min
            21, 255, 255, 255, 127, // i32_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_f32_as_fixed32() {
        // Arrange
        struct Message {
            f32_min: f32,
            f32_max: f32,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_f32_as_fixed32(&mut buffer, 1, self.f32_min);
                Self::encode_message_f32_as_fixed32(&mut buffer, 2, self.f32_max);

                Ok(buffer)
            }
        }

        let message = Message {
            f32_min: f32::MIN,
            f32_max: f32::MAX,
        };

        let expected_result: Vec<u8> = vec![
            13, 0, 0, 0, 0, // f32_min
            21, 255, 255, 255, 255, // f32_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_uint_as_fixed64() {
        // Arrange
        struct Message {
            u64_min: u64,
            u64_max: u64,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_uint_as_fixed64(&mut buffer, 1, self.u64_min);
                Self::encode_message_uint_as_fixed64(&mut buffer, 2, self.u64_max);

                Ok(buffer)
            }
        }

        let message = Message {
            u64_min: u64::MIN,
            u64_max: u64::MAX,
        };

        let expected_result: Vec<u8> = vec![
            9, 0, 0, 0, 0, 0, 0, 0, 0, // u64_min
            17, 255, 255, 255, 255, 255, 255, 255, 255, // u64_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_sint_as_sfixed64() {
        // Arrange
        struct Message {
            i64_min: i64,
            i64_max: i64,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_sint_as_sfixed64(&mut buffer, 1, self.i64_min);
                Self::encode_message_sint_as_sfixed64(&mut buffer, 2, self.i64_max);

                Ok(buffer)
            }
        }

        let message = Message {
            i64_min: i64::MIN,
            i64_max: i64::MAX,
        };

        let expected_result: Vec<u8> = vec![
            9, 0, 0, 0, 0, 0, 0, 0, 128, // i32_min
            17, 255, 255, 255, 255, 255, 255, 255, 127, // i32_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_f64_as_fixed64() {
        // Arrange
        struct Message {
            f64_min: f64,
            f64_max: f64,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_f64_as_fixed64(&mut buffer, 1, self.f64_min);
                Self::encode_message_f64_as_fixed64(&mut buffer, 2, self.f64_max);

                Ok(buffer)
            }
        }

        let message = Message {
            f64_min: f64::MIN,
            f64_max: f64::MAX,
        };

        let expected_result: Vec<u8> = vec![
            9, 0, 0, 0, 0, 0, 0, 0, 0, // f64_min
            17, 255, 255, 255, 255, 255, 255, 255, 255, // f64_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_byte_array_as_length_delimited() {
        // Arrange
        struct Message {
            byte_array: Vec<u8>,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_byte_array_as_ld(&mut buffer, 1, &self.byte_array);

                Ok(buffer)
            }
        }

        let message = Message {
            byte_array: vec![1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 2, 1],
        };

        let expected_result: Vec<u8> = vec![
            10, 15, 1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 2, 1, // byte_array
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_utf8_string_as_length_delimited() {
        // Arrange
        struct Message {
            string: String,
        }

        impl EncodeProtocolBufferTrait for Message {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_utf8_string_as_ld(&mut buffer, 1, &self.string);

                Ok(buffer)
            }
        }

        let message = Message {
            string: String::from("This is a test string."),
        };

        let expected_result: Vec<u8> = vec![
            10, 22, 84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 32, 115, 116,
            114, 105, 110, 103, 46, // string
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_sub_message() {
        // Arrange

        struct InnerMessage {
            test_inner_value1: u32,
            test_inner_value2: bool,
            test_inner_value3: f64,
            test_inner_value4: String,
        }

        impl EncodeProtocolBufferTrait for InnerMessage {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_uint_as_varint32(&mut buffer, 1, self.test_inner_value1);
                Self::encode_message_bool_as_varint32(&mut buffer, 2, self.test_inner_value2);
                Self::encode_message_f64_as_fixed64(&mut buffer, 3, self.test_inner_value3);
                Self::encode_message_utf8_string_as_ld(&mut buffer, 4, &self.test_inner_value4);

                Ok(buffer)
            }
        }

        struct OuterMessage {
            test_outer_value1: u64,
            test_outer_value2: String,
            inner: InnerMessage,
            test_outer_value4: i32,
        }

        impl EncodeProtocolBufferTrait for OuterMessage {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_uint_as_varint64(&mut buffer, 1, self.test_outer_value1);
                Self::encode_message_utf8_string_as_ld(&mut buffer, 2, &self.test_outer_value2);
                Self::encode_message(&mut buffer, 3, &self.inner)?;
                Self::encode_message_sint_as_sfixed32(&mut buffer, 4, self.test_outer_value4);

                Ok(buffer)
            }
        }

        let message = OuterMessage {
            test_outer_value1: 47623746278,
            test_outer_value2: "test outer".to_string(),
            inner: InnerMessage {
                test_inner_value1: 32,
                test_inner_value2: true,
                test_inner_value3: 1.567,
                test_inner_value4: "This inner".to_string(),
            },
            test_outer_value4: -1090,
        };

        let expected_result: Vec<u8> = vec![
            8, 230, 229, 226, 180, 177, 1, // test_outer_value1
            18, 10, 116, 101, 115, 116, 32, 111, 117, 116, 101, 114, // test_outer_value2
            26,  // Inner Message Tag
            25,  // Length
            8, 32, // test_inner_value1
            16, 1, // test_inner_value2
            25, 1, 0, 0, 0, 0, 0, 0, 0, // test_inner_value3
            34, 10, 84, 104, 105, 115, 32, 105, 110, 110, 101, 114, // test_inner_value4
            37, 190, 251, 255, 255, // test_outer_value4
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }

    #[test]
    fn test_encode_repeated_fields() {
        // Arrange

        struct InnerMessage {
            test_inner_value1: u32,
            test_inner_value2: Vec<bool>,
            test_inner_value3: f64,
            test_inner_value4: String,
        }

        impl EncodeProtocolBufferTrait for InnerMessage {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_uint_as_varint32(&mut buffer, 1, self.test_inner_value1);

                for value in &self.test_inner_value2 {
                    Self::encode_message_bool_as_varint32(&mut buffer, 2, *value);
                }

                Self::encode_message_f64_as_fixed64(&mut buffer, 3, self.test_inner_value3);
                Self::encode_message_utf8_string_as_ld(&mut buffer, 4, &self.test_inner_value4);

                Ok(buffer)
            }
        }

        struct OuterMessage {
            test_outer_value1: u64,
            test_outer_value2: String,
            inner: Vec<InnerMessage>,
            test_outer_value4: i32,
        }

        impl EncodeProtocolBufferTrait for OuterMessage {
            type Buffer = DataBuffer;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_uint_as_varint64(&mut buffer, 1, self.test_outer_value1);
                Self::encode_message_utf8_string_as_ld(&mut buffer, 2, &self.test_outer_value2);

                for i in 0..self.inner.len() {
                    Self::encode_message(&mut buffer, 3, &self.inner[i])?;
                }

                Self::encode_message_sint_as_sfixed32(&mut buffer, 4, self.test_outer_value4);

                Ok(buffer)
            }
        }

        let message = OuterMessage {
            test_outer_value1: 47623746278,
            test_outer_value2: "test outer".to_string(),
            inner: vec![
                InnerMessage {
                    test_inner_value1: 32,
                    test_inner_value2: vec![true, false],
                    test_inner_value3: 1.567,
                    test_inner_value4: "This inner".to_string(),
                },
                InnerMessage {
                    test_inner_value1: 28,
                    test_inner_value2: vec![true],
                    test_inner_value3: 1.0,
                    test_inner_value4: "This inner second message".to_string(),
                },
            ],
            test_outer_value4: -1090,
        };

        let expected_result: Vec<u8> = vec![
            8, 230, 229, 226, 180, 177, 1, // test_outer_value1
            18, 10, 116, 101, 115, 116, 32, 111, 117, 116, 101, 114, // test_outer_value2
            26,  // Inner Message Tag
            27,  // Length
            8, 32, // test_inner_value1
            16, 1, // test_inner_value2 - value 1
            16, 0, // test_inner_value2 - value 2
            25, 1, 0, 0, 0, 0, 0, 0, 0, // test_inner_value3
            34, 10, 84, 104, 105, 115, 32, 105, 110, 110, 101, 114, // test_inner_value4
            26,  // Inner Message Tag
            40,  // Length
            8, 28, // test_inner_value1
            16, 1, // test_inner_value2
            25, 1, 0, 0, 0, 0, 0, 0, 0, // test_inner_value2
            34, 25, 84, 104, 105, 115, 32, 105, 110, 110, 101, 114, 32, 115, 101, 99, 111, 110,
            100, 32, 109, 101, 115, 115, 97, 103, 101, // test_inner_value4
            37, 190, 251, 255, 255, // test_outer_value4
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result.as_bytes(), expected_result);
    }
}

use crate::{BufferTrait, WireType};

#[derive(Debug)]
pub struct EncodeProtocolBufferError;

pub trait EncodeProtocolBufferTrait {
    type Buffer: BufferTrait;

    /// Encodes encodes the associated structure to Protocol Buffers
    fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError>;

    /// Encodes a 8, 16 or 32 bit integer value (Signed or Unsigned)
    ///
    /// Note: Encoding of signed types is inefficient as they get expanded to the full 32 bits.
    /// Only use if the use of negative numbers is small.
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_int_as_varint32(buffer: &mut Self::Buffer, value: i32) {
        Self::encode_varint_32(buffer, value as u32);
    }

    /// Encodes a 8, 16 or 32 bit integer value (Signed or Unsigned) as a varint
    ///
    /// Note: Encoding of signed types is inefficient as they get expanded to the full 32 bits.
    /// Only use if the use of negative numbers is small.
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_int_as_varint32(buffer: &mut Self::Buffer, field_number: u32, value: i32) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_int_as_varint32(buffer, value);
    }

    /// Encodes a 8, 16, 32 or 64 bit integer value (Signed or Unsigned)
    ///
    /// Note: Encoding of signed types is inefficient as they get expanded to the full 64 bits.
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_int_as_varint64(buffer: &mut Self::Buffer, value: i64) {
        Self::encode_varint_64(buffer, value as u64);
    }

    /// Encodes a 8, 16, 32 or 64 bit integer value (Signed or Unsigned) as a varint
    ///
    /// Note: Encoding of signed types is inefficient as they get expanded to the full 64 bits.
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_int_as_varint64(buffer: &mut Self::Buffer, field_number: u32, value: i64) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_int_as_varint64(buffer, value);
    }

    /// Encodes a 8, 16 or 32 bit integer value (Unsigned only)
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_uint_as_varint32(buffer: &mut Self::Buffer, value: u32) {
        Self::encode_varint_32(buffer, value);
    }

    /// Encodes a 8, 16 or 32 bit integer value (Unsigned only) as a varint
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_uint_as_varint32(buffer: &mut Self::Buffer, field_number: u32, value: u32) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_uint_as_varint32(buffer, value);
    }

    /// Encodes a 8, 16, 32 or 64 bit integer value (Unsigned only)
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_uint_as_varint64(buffer: &mut Self::Buffer, value: u64) {
        Self::encode_varint_64(buffer, value);
    }

    /// Encodes a 8, 16, 32 or 64 bit integer value (Unsigned only) as a varint
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_uint_as_varint64(buffer: &mut Self::Buffer, field_number: u32, value: u64) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_uint_as_varint64(buffer, value);
    }

    /// Encodes a 8, 16 or 32 bit integer value (Signed only) with Zigzag encoding
    ///
    /// Note: Numbers either positive or negative closer to zero encode smaller.
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_sint_as_varint32_with_zigzag_encoding(buffer: &mut Self::Buffer, value: i32) {
        Self::encode_varint_32(buffer, Self::encode_zig_zag_32(value));
    }

    /// Encodes a 8, 16 or 32 bit integer value (Signed only) with Zigzag encoding as a varint
    ///
    /// Note: Numbers either positive or negative closer to zero encode smaller.
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_sint_as_varint32_with_zigzag_encoding(
        buffer: &mut Self::Buffer,
        field_number: u32,
        value: i32,
    ) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_sint_as_varint32_with_zigzag_encoding(buffer, value);
    }

    /// Encodes a 8, 16, 32 or 64 bit integer value (Signed only) with Zigzag encoding
    ///
    /// Note: Numbers either positive or negative closer to zero encode smaller.
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_sint_as_varint64_with_zigzag_encoding(buffer: &mut Self::Buffer, value: i64) {
        Self::encode_varint_64(buffer, Self::encode_zig_zag_64(value));
    }

    /// Encodes a 8, 16, 32 or 64 bit integer value (Signed only) with Zigzag encoding as a varint
    ///
    /// Note: Numbers either positive or negative closer to zero encode smaller.
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_sint_as_varint64_with_zigzag_encoding(
        buffer: &mut Self::Buffer,
        field_number: u32,
        value: i64,
    ) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_sint_as_varint64_with_zigzag_encoding(buffer, value);
    }

    /// Encodes a boolean value
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_bool_as_varint32(buffer: &mut Self::Buffer, value: bool) {
        Self::encode_varint_32(buffer, if value { 1 } else { 0 })
    }

    /// Encodes a boolean value as a varint
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_bool_as_varint32(buffer: &mut Self::Buffer, field_number: u32, value: bool) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_bool_as_varint32(buffer, value)
    }

    /// Encodes an enumeration value
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_enum_as_varint32(buffer: &mut Self::Buffer, value: u32) {
        Self::encode_varint_32(buffer, value)
    }

    /// Encodes an enumeration value as a varint
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_enum_as_varint32(buffer: &mut Self::Buffer, field_number: u32, value: u32) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_enum_as_varint32(buffer, value)
    }

    /// Encodes a value as a 32 bit unsigned integer in little endian
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_uint_as_fixed32(buffer: &mut Self::Buffer, value: u32) {
        Self::encode_byte(buffer, (value) as u8);
        Self::encode_byte(buffer, (value >> 8) as u8);
        Self::encode_byte(buffer, (value >> 16) as u8);
        Self::encode_byte(buffer, (value >> 24) as u8);
    }

    /// Encodes a value as a 32 bit unsigned integer in little endian as a fixed32
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_uint_as_fixed32(buffer: &mut Self::Buffer, field_number: u32, value: u32) {
        Self::encode_tag(buffer, field_number, WireType::Fixed32);
        Self::encode_value_uint_as_fixed32(buffer, value)
    }

    /// Encodes a value as a 32 bit signed integer in little endian
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_sint_as_sfixed32(buffer: &mut Self::Buffer, value: i32) {
        Self::encode_byte(buffer, (value) as u8);
        Self::encode_byte(buffer, (value >> 8) as u8);
        Self::encode_byte(buffer, (value >> 16) as u8);
        Self::encode_byte(buffer, (value >> 24) as u8);
    }

    /// Encodes a value as a 32 bit signed integer in little endian as an sfixed32
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_sint_as_sfixed32(buffer: &mut Self::Buffer, field_number: u32, value: i32) {
        Self::encode_tag(buffer, field_number, WireType::Fixed32);
        Self::encode_value_sint_as_sfixed32(buffer, value)
    }

    /// Encodes a 32 bit floating point value as a 32 bit in little endian
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_f32_as_fixed32(buffer: &mut Self::Buffer, value: f32) {
        Self::encode_value_uint_as_fixed32(buffer, value as u32);
    }

    /// Encodes a 32 bit floating point value as a 32 bit in little endian as an fixed32
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_f32_as_fixed32(buffer: &mut Self::Buffer, field_number: u32, value: f32) {
        Self::encode_tag(buffer, field_number, WireType::Fixed32);
        Self::encode_value_f32_as_fixed32(buffer, value)
    }

    /// Encodes a value as a 64 bit unsigned integer in little endian
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_uint_as_fixed64(buffer: &mut Self::Buffer, value: u64) {
        Self::encode_byte(buffer, (value) as u8);
        Self::encode_byte(buffer, (value >> 8) as u8);
        Self::encode_byte(buffer, (value >> 16) as u8);
        Self::encode_byte(buffer, (value >> 24) as u8);
        Self::encode_byte(buffer, (value >> 32) as u8);
        Self::encode_byte(buffer, (value >> 40) as u8);
        Self::encode_byte(buffer, (value >> 48) as u8);
        Self::encode_byte(buffer, (value >> 56) as u8);
    }

    /// Encodes a value as a 64 bit unsigned integer in little endian as a fixed32
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_uint_as_fixed64(buffer: &mut Self::Buffer, field_number: u32, value: u64) {
        Self::encode_tag(buffer, field_number, WireType::Fixed64);
        Self::encode_value_uint_as_fixed64(buffer, value)
    }

    /// Encodes a value as a 64 bit signed integer in little endian
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_sint_as_sfixed64(buffer: &mut Self::Buffer, value: i64) {
        Self::encode_byte(buffer, (value) as u8);
        Self::encode_byte(buffer, (value >> 8) as u8);
        Self::encode_byte(buffer, (value >> 16) as u8);
        Self::encode_byte(buffer, (value >> 24) as u8);
        Self::encode_byte(buffer, (value >> 32) as u8);
        Self::encode_byte(buffer, (value >> 40) as u8);
        Self::encode_byte(buffer, (value >> 48) as u8);
        Self::encode_byte(buffer, (value >> 56) as u8);
    }

    /// Encodes a value as a 64 bit signed integer in little endian as a sfixed32
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_sint_as_sfixed64(buffer: &mut Self::Buffer, field_number: u32, value: i64) {
        Self::encode_tag(buffer, field_number, WireType::Fixed64);
        Self::encode_value_sint_as_sfixed64(buffer, value)
    }

    /// Encodes a 64 bit floating point value as a 32 bit in little endian
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_f64_as_fixed64(buffer: &mut Self::Buffer, value: f64) {
        Self::encode_value_uint_as_fixed64(buffer, value as u64);
    }

    /// Encodes a 64 bit floating point value as a 32 bit in little endian as an fixed64
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_f64_as_fixed64(buffer: &mut Self::Buffer, field_number: u32, value: f64) {
        Self::encode_tag(buffer, field_number, WireType::Fixed64);
        Self::encode_value_f64_as_fixed64(buffer, value)
    }

    /// Encodes a byte array
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_byte_array_as_ld(buffer: &mut Self::Buffer, value: &[u8]) {
        Self::encode_length(buffer, value.len());
        for byte in value {
            Self::encode_byte(buffer, *byte);
        }
    }

    /// Encodes a byte array as a length delimited
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_byte_array_as_ld(buffer: &mut Self::Buffer, field_number: u32, value: &[u8]) {
        Self::encode_tag(buffer, field_number, WireType::LengthDelimited);
        Self::encode_value_byte_array_as_ld(buffer, value);
    }

    /// Encodes a utf8 string
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_value_utf8_string_as_ld(buffer: &mut Self::Buffer, value: &str) {
        Self::encode_value_byte_array_as_ld(buffer, value.as_bytes());
    }

    /// Encodes a utf8 string as a length delimited
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message_utf8_string_as_ld(buffer: &mut Self::Buffer, field_number: u32, value: &str) {
        Self::encode_tag(buffer, field_number, WireType::LengthDelimited);
        Self::encode_value_utf8_string_as_ld(buffer, value);
    }

    /// Encodes a sub message as a length delimited
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'value' - The value to be encoded.
    fn encode_message(
        buffer: &mut Self::Buffer,
        field_number: u32,
        value: &impl EncodeProtocolBufferTrait,
    ) -> Result<(), EncodeProtocolBufferError> {
        let temp_buffer = value.encode_protocol_buffer()?;
        let encoded_bytes = temp_buffer.as_bytes();

        Self::encode_message_byte_array_as_ld(buffer, field_number, encoded_bytes);

        Ok(())
    }

    /// Encodes a single byte
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_byte(buffer: &mut Self::Buffer, value: u8) {
        buffer.add_byte_to_end(value);
    }

    /// Encodes a tag
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'field_number' - The field number to assign the value.
    /// * 'wire_type' - The type of data encoding the tag represents.
    fn encode_tag(buffer: &mut Self::Buffer, field_number: u32, wire_type: WireType) {
        Self::encode_varint_32(buffer, Self::make_tag(field_number, wire_type));
    }

    /// Encodes a length
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'length' - The length of the data to follow.
    fn encode_length(buffer: &mut Self::Buffer, length: usize) {
        Self::encode_varint_32(buffer, length as u32);
    }

    /// Encodes a Wire Type and Field number into a Tag
    ///
    /// # Arguments
    ///
    /// * 'field_number' - The field number to assign the value.
    /// * 'wire_type' - The type of data encoding the tag represents.
    ///
    /// Returns the encoded tag.
    fn make_tag(field_number: u32, wire_type: WireType) -> u32 {
        let wt = wire_type as u32;

        (((field_number << 3) as u32) | wt) as u32
    }

    /// Encodes a 32 bit value into a varint32
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_varint_32(buffer: &mut Self::Buffer, value: u32) {
        Self::encode_varint_64(buffer, value as u64);
    }

    /// Encodes a 64 bit value into a varint32
    ///
    /// # Arguments
    ///
    /// * 'buffer' - A buffer object to append the new encoding too.
    /// * 'value' - The value to be encoded.
    fn encode_varint_64(buffer: &mut Self::Buffer, mut value: u64) {
        if value < 128 {
            buffer.add_byte_to_end(value as u8);
            return;
        }

        loop {
            if value > 127 {
                buffer.add_byte_to_end(((value & 0x7F) | 0x80) as u8);
                value >>= 7;
            } else {
                buffer.add_byte_to_end(value as u8);
                return;
            }
        }
    }

    /// Encodes a 32 value using the zigzag algorithm
    ///
    /// # Arguments
    ///
    /// * 'value' - The value to be encoded.
    ///
    /// Return the encoding
    fn encode_zig_zag_32(value: i32) -> u32 {
        ((value << 1) ^ (value >> 31)) as u32
    }

    /// Encodes a 64 value using the zigzag algorithm
    ///
    /// # Arguments
    ///
    /// * 'value' - The value to be encoded.
    ///
    /// Return the encoding
    fn encode_zig_zag_64(value: i64) -> u64 {
        ((value << 1) ^ (value >> 63)) as u64
    }
}
