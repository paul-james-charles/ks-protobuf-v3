use crate::{Buffer, DecodeError, LengthDelimited, Varint};

#[derive(Debug, Default, Eq, PartialEq)]
struct Length(u32);

impl From<usize> for Length {
    fn from(length: usize) -> Self {
        Self(length as u32)
    }
}

impl Varint for Length {
    fn to_varint(&self, buffer: &mut Buffer) -> usize {
        self.0.to_varint(buffer)
    }

    fn from_varint(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        self.0.from_varint(buffer)
    }
}

impl LengthDelimited for Vec<u8> {
    fn to_length_delimited(&self, buffer: &mut Buffer) -> usize {
        let length = Length::from(self.len());
        let mut size = length.to_varint(buffer);

        for b in self {
            buffer.put_u8(*b);
            size += 1;
        }

        size
    }

    fn from_length_delimited(&mut self, buffer: &[u8]) -> Result<usize, DecodeError> {
        let mut length = Length::default();

        let size1 = length.from_varint(buffer)?;
        for b in &buffer[size1..size1 + length.0 as usize] {
            self.push(*b);
        }

        Ok(size1 + length.0 as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(Length(1), vec![1])]
    #[case(Length(1024), vec![128, 8])]
    #[case(Length(65535), vec![255, 255, 3])]
    fn test_varint32_encoding(#[case] length: Length, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = length.to_varint(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![1], Length(1), 1)]
    #[case(vec![128, 8], Length(1024), 2)]
    #[case(vec![255, 255, 3], Length(65535), 3)]
    fn test_varint32_decoding(
        #[case] bytes: Vec<u8>,
        #[case] expected_length: Length,
        #[case] expected_size: usize,
    ) {
        let mut length = Length(0);

        // Act
        let size = length.from_varint(&bytes).unwrap();

        // Assert
        assert_eq!(length, expected_length);
        assert_eq!(size, expected_size);
    }

    #[rstest]
    #[case(vec![1, 2, 3, 4], vec![4, 1, 2, 3, 4])]
    #[case(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![8, 1, 2, 3, 4, 5, 6, 7, 8])]
    fn test_byte_array_encoding(#[case] bytes: Vec<u8>, #[case] expected_buffer: Vec<u8>) {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        let size = bytes.to_length_delimited(&mut buffer);

        // Assert
        assert_eq!(buffer.to_vec(), expected_buffer);
        assert_eq!(size, expected_buffer.len());
    }

    #[rstest]
    #[case(vec![4, 1, 2, 3, 4], vec![1, 2, 3, 4], 5)]
    #[case(vec![8, 1, 2, 3, 4, 5, 6, 7, 8], vec![1, 2, 3, 4, 5, 6, 7, 8], 9)]
    fn test_byte_array_decoding(
        #[case] bytes: Vec<u8>,
        #[case] expected_value: Vec<u8>,
        #[case] expected_size: usize,
    ) {
        let mut byte_array: Vec<u8> = Vec::new();

        // Act
        let size = byte_array.from_length_delimited(&bytes).unwrap();

        // Assert
        assert_eq!(byte_array, expected_value);
        assert_eq!(size, expected_size);
    }
}
