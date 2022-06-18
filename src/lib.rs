#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    impl BufferTrait for Vec<u8> {
        fn push(&mut self, value: u8) {
            self.push(value);
        }
    }

    #[test]
    fn test_encode_varint32_int32_to_protocol_buffer() {
        // Arrange
        struct VarintInt32Message {
            varint32_i8_min: i8,
            varint32_i8_max: i8,
            varint32_i16_min: i16,
            varint32_i16_max: i16,
            varint32_i32_min: i32,
            varint32_i32_max: i32,
            varint32_u8_min: u8,
            varint32_u8_max: u8,
            varint32_u16_min: u16,
            varint32_u16_max: u16,
            varint32_u32_min: u32,
            varint32_u32_max: u32,
        }

        impl EncodeProtocolBufferTrait for VarintInt32Message {
            type Buffer = Vec<u8>;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();
                Self::encode_message_varint_int32(&mut buffer, 1, self.varint32_i8_min as i32);
                Self::encode_message_varint_int32(&mut buffer, 2, self.varint32_i8_max as i32);
                Self::encode_message_varint_int32(&mut buffer, 3, self.varint32_i16_min as i32);
                Self::encode_message_varint_int32(&mut buffer, 4, self.varint32_i16_max as i32);
                Self::encode_message_varint_int32(&mut buffer, 5, self.varint32_i32_min as i32);
                Self::encode_message_varint_int32(&mut buffer, 6, self.varint32_i32_max as i32);

                Self::encode_message_varint_int32(&mut buffer, 7, self.varint32_u8_min as i32);
                Self::encode_message_varint_int32(&mut buffer, 8, self.varint32_u8_max as i32);
                Self::encode_message_varint_int32(&mut buffer, 9, self.varint32_u16_min as i32);
                Self::encode_message_varint_int32(&mut buffer, 10, self.varint32_u16_max as i32);
                Self::encode_message_varint_int32(&mut buffer, 11, self.varint32_u32_min as i32);
                Self::encode_message_varint_int32(&mut buffer, 12, self.varint32_u32_max as i32);

                Ok(buffer)
            }
        }


        let message = VarintInt32Message {
            varint32_i8_min: i8::MIN,
            varint32_i8_max: i8::MAX,
            varint32_i16_min: i16::MIN,
            varint32_i16_max: i16::MAX,
            varint32_i32_min: i32::MIN,
            varint32_i32_max: i32::MAX,
            varint32_u8_min: u8::MIN,
            varint32_u8_max: u8::MAX,
            varint32_u16_min: u16::MIN,
            varint32_u16_max: u16::MAX,
            varint32_u32_min: u32::MIN,
            varint32_u32_max: u32::MAX,
        };

        let expected_result: Vec<u8> = vec![
            8, 128, 255, 255, 255, 15, // varint32_i8_min
            16, 127, // varint32_i8_max
            24, 128, 128, 254, 255, 15, // varint32_i16_min
            32, 255, 255, 1, // varint32_i16_max
            40, 128, 128, 128, 128, 8, // varint32_i32_min
            48, 255, 255, 255, 255, 7, // varint32_i32_max
            56, 0, // varint32_u8_min
            64, 255, 1, // varint32_u8_max
            72, 0, // varint32_u16_min
            80, 255, 255, 3, // varint32_u16_max
            88, 0, // varint32_u32_min
            96, 255, 255, 255, 255, 15, // varint32_u32_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_encode_varint32_uint32_to_protocol_buffer() {
        // Arrange
        struct VarintUInt32Message {
            varint32_u8_min: u8,
            varint32_u8_max: u8,
            varint32_u16_min: u16,
            varint32_u16_max: u16,
            varint32_u32_min: u32,
            varint32_u32_max: u32,
        }

        impl EncodeProtocolBufferTrait for VarintUInt32Message {
            type Buffer = Vec<u8>;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_varint_uint32(&mut buffer, 1, self.varint32_u8_min as u32);
                Self::encode_message_varint_uint32(&mut buffer, 2, self.varint32_u8_max as u32);
                Self::encode_message_varint_uint32(&mut buffer, 3, self.varint32_u16_min as u32);
                Self::encode_message_varint_uint32(&mut buffer, 4, self.varint32_u16_max as u32);
                Self::encode_message_varint_uint32(&mut buffer, 5, self.varint32_u32_min);
                Self::encode_message_varint_uint32(&mut buffer, 6, self.varint32_u32_max);

                Ok(buffer)
            }
        }

        let message = VarintUInt32Message {
            varint32_u8_min: u8::MIN,
            varint32_u8_max: u8::MAX,
            varint32_u16_min: u16::MIN,
            varint32_u16_max: u16::MAX,
            varint32_u32_min: u32::MIN,
            varint32_u32_max: u32::MAX,
        };

        let expected_result: Vec<u8> = vec![
            8, 0, // varint32_u8_min
            16, 255, 1, // varint32_u8_max
            24, 0, // varint32_u16_min
            32, 255, 255, 3, // varint32_u16_max
            40, 0, // varint32_u32_min
            48, 255, 255, 255, 255, 15, // varint32_u32_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_encode_varint64_int64_to_protocol_buffer() {
        // Arrange
        struct VarintInt64Message {
            varint64_i8_min: i8,
            varint64_i8_max: i8,
            varint64_i16_min: i16,
            varint64_i16_max: i16,
            varint64_i32_min: i32,
            varint64_i32_max: i32,
            varint64_i64_min: i64,
            varint64_i64_max: i64,
            varint64_u8_min: u8,
            varint64_u8_max: u8,
            varint64_u16_min: u16,
            varint64_u16_max: u16,
            varint64_u32_min: u32,
            varint64_u32_max: u32,
            varint64_u64_min: u64,
            varint64_u64_max: u64,
        }

        impl EncodeProtocolBufferTrait for VarintInt64Message {
            type Buffer = Vec<u8>;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();
                Self::encode_message_varint_int64(&mut buffer, 1, self.varint64_i8_min as i64);
                Self::encode_message_varint_int64(&mut buffer, 2, self.varint64_i8_max as i64);
                Self::encode_message_varint_int64(&mut buffer, 3, self.varint64_i16_min as i64);
                Self::encode_message_varint_int64(&mut buffer, 4, self.varint64_i16_max as i64);
                Self::encode_message_varint_int64(&mut buffer, 5, self.varint64_i32_min as i64);
                Self::encode_message_varint_int64(&mut buffer, 6, self.varint64_i32_max as i64);
                Self::encode_message_varint_int64(&mut buffer, 7, self.varint64_i64_min as i64);
                Self::encode_message_varint_int64(&mut buffer, 8, self.varint64_i64_max as i64);

                Self::encode_message_varint_int64(&mut buffer, 9, self.varint64_u8_min as i64);
                Self::encode_message_varint_int64(&mut buffer, 10, self.varint64_u8_max as i64);
                Self::encode_message_varint_int64(&mut buffer, 11, self.varint64_u16_min as i64);
                Self::encode_message_varint_int64(&mut buffer, 12, self.varint64_u16_max as i64);
                Self::encode_message_varint_int64(&mut buffer, 13, self.varint64_u32_min as i64);
                Self::encode_message_varint_int64(&mut buffer, 14, self.varint64_u32_max as i64);
                Self::encode_message_varint_int64(&mut buffer, 15, self.varint64_u64_min as i64);
                Self::encode_message_varint_int64(&mut buffer, 16, self.varint64_u64_max as i64);

                Ok(buffer)
            }
        }

        let message = VarintInt64Message {
            varint64_i8_min: i8::MIN,
            varint64_i8_max: i8::MAX,
            varint64_i16_min: i16::MIN,
            varint64_i16_max: i16::MAX,
            varint64_i32_min: i32::MIN,
            varint64_i32_max: i32::MAX,
            varint64_i64_min: i64::MIN,
            varint64_i64_max: i64::MAX,
            varint64_u8_min: u8::MIN,
            varint64_u8_max: u8::MAX,
            varint64_u16_min: u16::MIN,
            varint64_u16_max: u16::MAX,
            varint64_u32_min: u32::MIN,
            varint64_u32_max: u32::MAX,
            varint64_u64_min: u64::MIN,
            varint64_u64_max: u64::MAX,
        };

        let expected_result: Vec<u8> = vec![
            8, 128, 255, 255, 255, 255, 255, 255, 255, 255, 1, // varint64_i8_min
            16, 127, // varint64_i8_max
            24, 128, 128, 254, 255, 255, 255, 255, 255, 255, 1, // varint64_i16_min
            32, 255, 255, 1, // varint64_i16_max
            40, 128, 128, 128, 128, 248, 255, 255, 255, 255, 1, // varint64_i32_min
            48, 255, 255, 255, 255, 7, // varint64_i32_max
            56, 128, 128, 128, 128, 128, 128, 128, 128, 128, 1, // varint64_u64_min
            64, 255, 255, 255, 255, 255, 255, 255, 255, 127, // varint64_u64_max
            72, 0, // varint64_u8_min
            80, 255, 1, // varint64_u8_max
            88, 0, // varint64_u16_min
            96, 255, 255, 3, // varint64_u16_max
            104, 0, // varint64_u32_min
            112, 255, 255, 255, 255, 15, // varint64_u32_max
            120, 0, // varint64_u64_min
            128, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, // varint64_u64_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_encode_varint64_uint64_to_protocol_buffer() {
        // Arrange
        struct VarintUInt64Message {
            varint64_u8_min: u8,
            varint64_u8_max: u8,
            varint64_u16_min: u16,
            varint64_u16_max: u16,
            varint64_u32_min: u32,
            varint64_u32_max: u32,
            varint64_u64_min: u64,
            varint64_u64_max: u64,
        }

        impl EncodeProtocolBufferTrait for VarintUInt64Message {
            type Buffer = Vec<u8>;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_varint_uint64(&mut buffer, 1, self.varint64_u8_min as u64);
                Self::encode_message_varint_uint64(&mut buffer, 2, self.varint64_u8_max as u64);
                Self::encode_message_varint_uint64(&mut buffer, 3, self.varint64_u16_min as u64);
                Self::encode_message_varint_uint64(&mut buffer, 4, self.varint64_u16_max as u64);
                Self::encode_message_varint_uint64(&mut buffer, 5, self.varint64_u32_min as u64);
                Self::encode_message_varint_uint64(&mut buffer, 6, self.varint64_u32_max as u64);
                Self::encode_message_varint_uint64(&mut buffer, 7, self.varint64_u64_min as u64);
                Self::encode_message_varint_uint64(&mut buffer, 8, self.varint64_u64_max as u64);

                Ok(buffer)
            }
        }

        let message = VarintUInt64Message {
            varint64_u8_min: u8::MIN,
            varint64_u8_max: u8::MAX,
            varint64_u16_min: u16::MIN,
            varint64_u16_max: u16::MAX,
            varint64_u32_min: u32::MIN,
            varint64_u32_max: u32::MAX,
            varint64_u64_min: u64::MIN,
            varint64_u64_max: u64::MAX,
        };

        let expected_result: Vec<u8> = vec![
            8, 0, // varint64_u8_min
            16, 255, 1, // varint64_u8_max
            24, 0, // varint64_u16_min
            32, 255, 255, 3, // varint64_u16_max
            40, 0, // varint64_u32_min
            48, 255, 255, 255, 255, 15, // varint64_u32_max
            56, 0, // varint64_u64_min
            64, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1 // varint64_u64_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_encode_varint32_sint32_to_protocol_buffer() {
        // Arrange
        struct VarintSInt32Message {
            varint32_s8_min: i8,
            varint32_s8_max: i8,
            varint32_s16_min: i16,
            varint32_s16_max: i16,
            varint32_s32_min: i32,
            varint32_s32_max: i32,
        }

        impl EncodeProtocolBufferTrait for VarintSInt32Message {
            type Buffer = Vec<u8>;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_varint_sint32(&mut buffer, 1, self.varint32_s8_min as i32);
                Self::encode_message_varint_sint32(&mut buffer, 2, self.varint32_s8_max as i32);
                Self::encode_message_varint_sint32(&mut buffer, 3, self.varint32_s16_min as i32);
                Self::encode_message_varint_sint32(&mut buffer, 4, self.varint32_s16_max as i32);
                Self::encode_message_varint_sint32(&mut buffer, 5, self.varint32_s32_min);
                Self::encode_message_varint_sint32(&mut buffer, 6, self.varint32_s32_max);

                Ok(buffer)
            }
        }

        let message = VarintSInt32Message {
            varint32_s8_min: i8::MIN,
            varint32_s8_max: i8::MAX,
            varint32_s16_min: i16::MIN,
            varint32_s16_max: i16::MAX,
            varint32_s32_min: i32::MIN,
            varint32_s32_max: i32::MAX,
        };

        let expected_result: Vec<u8> = vec![
            8, 255, 1, // varint32_s8_min
            16, 254, 1, // varint32_s8_max
            24, 255, 255, 3, // varint32_s16_min
            32, 254, 255, 3, // varint32_s16_max
            40, 255, 255, 255, 255, 15, // varint32_s32_min
            48, 254, 255, 255, 255, 15, // varint32_s32_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_encode_varint64_sint64_to_protocol_buffer() {
        // Arrange
        struct VarintSInt64Message {
            varint64_s8_min: i8,
            varint64_s8_max: i8,
            varint64_s16_min: i16,
            varint64_s16_max: i16,
            varint64_s32_min: i32,
            varint64_s32_max: i32,
            varint64_s64_min: i64,
            varint64_s64_max: i64,
        }

        impl EncodeProtocolBufferTrait for VarintSInt64Message {
            type Buffer = Vec<u8>;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_varint_sint64(&mut buffer, 1, self.varint64_s8_min as i64);
                Self::encode_message_varint_sint64(&mut buffer, 2, self.varint64_s8_max as i64);
                Self::encode_message_varint_sint64(&mut buffer, 3, self.varint64_s16_min as i64);
                Self::encode_message_varint_sint64(&mut buffer, 4, self.varint64_s16_max as i64);
                Self::encode_message_varint_sint64(&mut buffer, 5, self.varint64_s32_min as i64);
                Self::encode_message_varint_sint64(&mut buffer, 6, self.varint64_s32_max as i64);
                Self::encode_message_varint_sint64(&mut buffer, 7, self.varint64_s64_min);
                Self::encode_message_varint_sint64(&mut buffer, 8, self.varint64_s64_max);

                Ok(buffer)
            }
        }

        let message = VarintSInt64Message {
            varint64_s8_min: i8::MIN,
            varint64_s8_max: i8::MAX,
            varint64_s16_min: i16::MIN,
            varint64_s16_max: i16::MAX,
            varint64_s32_min: i32::MIN,
            varint64_s32_max: i32::MAX,
            varint64_s64_min: i64::MIN,
            varint64_s64_max: i64::MAX,
        };

        let expected_result: Vec<u8> = vec![
            8, 255, 1, // varint64_s8_min
            16, 254, 1, // varint64_s8_max
            24, 255, 255, 3, // varint64_s16_min
            32, 254, 255, 3, // varint64_s16_max
            40, 255, 255, 255, 255, 15, // varint64_s32_min
            48, 254, 255, 255, 255, 15, // varint=64_s32_max
            56, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, // varint64_s64_min
            64, 254, 255, 255, 255, 255, 255, 255, 255, 255, 1, // varint64_s64_max
        ];

        // Act
        let result = message.encode_protocol_buffer().unwrap();

        // Assert
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_encode_varint_misc_to_protocol_buffer() {
        // Arrange
        struct VarintMiscMessage {
            bool_true: bool,
            bool_false: bool,
            enum_value_1: u32,
            enum_value_2: u32,
        }

        impl EncodeProtocolBufferTrait for VarintMiscMessage {
            type Buffer = Vec<u8>;

            fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError> {
                let mut buffer = Self::Buffer::new();

                Self::encode_message_varint_bool(&mut buffer, 1, self.bool_true);
                Self::encode_message_varint_bool(&mut buffer, 2, self.bool_false);
                Self::encode_message_varint_enum(&mut buffer, 3, self.enum_value_1);
                Self::encode_message_varint_enum(&mut buffer, 4, self.enum_value_2);

                Ok(buffer)
            }
        }

        let message = VarintMiscMessage {
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
        assert_eq!(result, expected_result);
    }

}

pub enum WireType {
    Varint = 0,
    Fixed64,
    LengthDelimited,
    StartGroup,
    EndGroup,
    Fixed32,
}

#[derive(Debug)]
pub struct EncodeProtocolBufferError;

pub trait BufferTrait {
    fn push(&mut self, value: u8);
}

pub trait EncodeProtocolBufferTrait {
    type Buffer: BufferTrait;

    fn encode_protocol_buffer(&self) -> Result<Self::Buffer, EncodeProtocolBufferError>;

    // fn encode_message_global_start(&mut self, field_number: u32) {
    //     self.encode_tag(field_number, WireType::StartGroup);
    // }
    //
    // fn encode_message_global_end(&mut self, field_number: u32) {
    //     self.encode_tag(field_number, WireType::EndGroup);
    // }
    //
    fn encode_value_varint_int32(buffer: &mut Self::Buffer, value: i32) {
        Self::encode_varint_32(buffer, value as u32);
    }

    fn encode_message_varint_int32(buffer: &mut Self::Buffer, field_number: u32, value: i32) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_varint_int32(buffer, value);
    }

    fn encode_value_varint_int64(buffer: &mut Self::Buffer, value: i64) {
        Self::encode_varint_64(buffer, value as u64);
    }

    fn encode_message_varint_int64(buffer: &mut Self::Buffer, field_number: u32, value: i64) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_varint_int64(buffer, value);
    }

    fn encode_value_varint_uint32(buffer: &mut Self::Buffer,  value: u32) {
        Self::encode_varint_32(buffer, value);
    }

    fn encode_message_varint_uint32(buffer: &mut Self::Buffer, field_number: u32, value: u32) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_varint_uint32(buffer, value);
    }

    fn encode_value_varint_uint64(buffer: &mut Self::Buffer, value: u64) {
        Self::encode_varint_64(buffer, value);
    }

    fn encode_message_varint_uint64(buffer: &mut Self::Buffer, field_number: u32, value: u64) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_varint_uint64(buffer, value);
    }

    fn encode_value_varint_sint32(buffer: &mut Self::Buffer, value: i32) {
        Self::encode_varint_32(buffer, Self::encode_zig_zag_32(value));
    }

    fn encode_message_varint_sint32(buffer: &mut Self::Buffer, field_number: u32, value: i32) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_varint_sint32(buffer, value);
    }

    fn encode_value_varint_sint64(buffer: &mut Self::Buffer, value: i64) {
        Self::encode_varint_64(buffer, Self::encode_zig_zag_64(value));
    }

    fn encode_message_varint_sint64(buffer: &mut Self::Buffer, field_number: u32, value: i64) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_varint_sint64(buffer, value);
    }

    fn encode_value_varint_bool(buffer: &mut Self::Buffer, value: bool) {
        Self::encode_varint_32(buffer, if value == true { 1 } else { 0 })
    }

    fn encode_message_varint_bool(buffer: &mut Self::Buffer, field_number: u32, value: bool) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_varint_bool(buffer, value)
    }

    fn encode_value_varint_enum(buffer: &mut Self::Buffer, value: u32) {
        Self::encode_varint_32(buffer, value)
    }

    fn encode_message_varint_enum(buffer: &mut Self::Buffer, field_number: u32, value: u32) {
        Self::encode_tag(buffer, field_number, WireType::Varint);
        Self::encode_value_varint_enum(buffer, value)
    }

    //     fn append_fixed32bit_sfixed32(&mut self, field_number: u32, value: i32) {}
    //     fn append_fixed32bit_fixed32(&mut self, field_number: u32, value: i64) {}
    //     fn append_fixed32bit_float(&mut self, field_number: u32, value: i64) {}
    //     fn append_fixed64bit_sfixed64(&mut self, field_number: u32, value: i32) {}
    //     fn append_fixed64bit_fixed64(&mut self, field_number: u32, value: i64) {}
    //     fn append_fixed64bit_double(&mut self, field_number: u32, value: i64) {}
    //     fn append_delimited_message(&mut self, field_number: u32, value: i64) {}
    //     fn append_delimited_string(&mut self, field_number: u32, value: i64) {}
    //     fn append_delimited_bytes(&mut self, field_number: u32, value: i64) {}
    //     fn append_delimited_packed_varint(&mut self, field_number: u32, value: i64) {}
    //     fn append_delimited_packed_fixed32bit(&mut self, field_number: u32, value: i64) {}
    //     fn append_delimited_packed_fixed64bit(&mut self, field_number: u32, value: i64) {}

    fn encode_tag(buffer: &mut Self::Buffer, field_number: u32, wire_type: WireType) {
        Self::encode_varint_32(buffer, Self::make_tag(field_number, wire_type));
    }

    fn make_tag(field_number: u32, wire_type: WireType) -> u32 {
        let wt = wire_type as u32;

        (((field_number << 3) as u32) | wt) as u32
    }

    fn encode_varint_32(buffer: &mut Self::Buffer, value: u32) {
        Self::encode_varint_64(buffer, value as u64);
    }

    fn encode_varint_64(buffer: &mut Self::Buffer, mut value: u64) {
        if value < 128 {
            buffer.push(value as u8);
            return;
        }

        loop {
            if value > 127 {
                buffer.push(((value & 0x7F) | 0x80) as u8);
                value >>= 7;
            } else {
                buffer.push(value as u8);
                return;
            }
        }
    }

    fn encode_zig_zag_32(value: i32) -> u32 {
        ((value << 1) ^ (value >> 31)) as u32
    }

    fn encode_zig_zag_64(value: i64) -> u64 {
        ((value << 1) ^ (value >> 63)) as u64
    }
}


