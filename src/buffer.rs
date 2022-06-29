/// Buffer used to store Protocol Buffer encoded data.
///
/// The structure will dynamically grow when required.
#[derive(Default, Debug)]
pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    /// Obtains a new Vec<u8> of the buffer. The information is cloned by default.
    pub fn to_vec(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// Appends a single byte to the end of the buffer. The buffer will grow if necessary.
    pub fn put_u8(&mut self, value: u8) {
        self.data.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_of_new_buffer() {
        // Act
        let buffer = Buffer::default();

        // Assert
        assert_eq!(buffer.to_vec(), vec![]);
    }

    #[test]
    fn test_addition_of_byte_to_buffer() {
        // Arrange
        let mut buffer = Buffer::default();

        // Act
        buffer.put_u8(10);
        buffer.put_u8(5);

        // Assert
        assert_eq!(buffer.to_vec(), vec![10, 5]);
    }
}
