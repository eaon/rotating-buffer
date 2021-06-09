pub struct RotatingBuffer<T: Default + Copy, const S: usize> {
    inner_length: usize,
    inner: [T; S],
    rotated_length: usize,
}

impl<T: Default + Copy, const S: usize> RotatingBuffer<T, S> {
    pub fn new() -> RotatingBuffer<T, S> {
        RotatingBuffer {
            inner: [T::default(); S],
            inner_length: 0,
            rotated_length: 0,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner[..self.inner_length]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.inner[self.rotated_length..]
    }

    pub fn is_empty(&self) -> bool {
        self.inner_length == 0
    }

    pub fn len(&self) -> usize {
        self.inner_length
    }

    pub fn set_len(&mut self, length: usize) {
        if length > S {
            panic!("Input length larger than buffer capacity: {} > {}", length, S);
        }

        self.inner_length = self.rotated_length + length;
        self.rotated_length = 0;
    }

    pub fn rotate_starting_at(&mut self, index: usize) {
        if index > self.inner_length {
            panic!("Invalid index: {} > {}", index, self.inner_length);
        }

        let rotation_length = self.inner_length - index;
        self.inner[..self.inner_length].rotate_right(rotation_length);
        self.rotated_length = rotation_length;
        self.inner_length = rotation_length;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_through() {
        let mut buf = RotatingBuffer::<u8, 32>::new();
        println!("Original length: 0");
        assert_eq!(0, buf.len());
        println!("Writable buffer size, flexible, but right now: 32");
        assert_eq!(32, buf.as_mut_slice().len());
        // Let's write! More or less compatible with the way UnixStream::try_read works
        buf.as_mut_slice().copy_from_slice(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        // Amount of bytes read as returned by UnixStream::try_read
        buf.set_len(22);
        println!("Buffer contents: {:?}", buf.as_slice());
        // Post read, we overflow at index 17
        buf.rotate_starting_at(17);
        // New length
        assert_eq!(5, buf.len());
        println!("Buffer contents: {:?}", buf.as_slice());
        // Write a full message to the buffer! 27 new spicey bytes
        buf.as_mut_slice().copy_from_slice(&[
            22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
            44, 45, 46, 47, 48
        ]);
        buf.set_len(24);
        println!(
            "Wrote 24 new bytes to buffer\nNew buffer contents: {:?}",
            buf.as_slice()
        );
        assert_eq!(29, buf.len());
        // Overflow again at 21
        buf.rotate_starting_at(21);
        assert_eq!(8, buf.len());
        // More new data
        buf.as_mut_slice().copy_from_slice(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        // And the data length
        buf.set_len(16);
        println!(
            "Wrote 16 new bytes to buffer\nNew buffer contents: {:?}",
            buf.as_slice()
        );
        assert_eq!(24, buf.len());
    }
}
