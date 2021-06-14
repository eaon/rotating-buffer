#![no_std]

#[derive(Clone, Copy)]
pub struct RotatingBuffer<T: Default + Copy, const S: usize> {
    inner_length: usize,
    pub inner: [T; S],
}

impl<T: Default + Copy, const S: usize> RotatingBuffer<T, S> {
    pub fn new() -> RotatingBuffer<T, S> {
        RotatingBuffer {
            inner: [T::default(); S],
            inner_length: 0,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner[..self.inner_length]
    }

    pub fn append_only_mut_slice(&mut self) -> &mut [T] {
        &mut self.inner[self.inner_length..]
    }

    pub fn is_empty(&self) -> bool {
        self.inner_length == 0
    }

    pub fn len(&self) -> usize {
        self.inner_length
    }

    pub fn capacity(&self) -> usize {
        S
    }

    pub fn resize(&mut self, new_len: usize) {
        assert!(new_len <= S);
        self.inner_length = new_len;
    }

    pub fn add_len(&mut self, new_len: usize) -> usize {
        self.resize(self.inner_length + new_len);
        self.inner_length
    }

    pub fn rotate_right(&mut self, k: usize) {
        self.inner[..self.inner_length].rotate_right(k);
        self.inner_length = k;
    }

    pub fn rotate_right_at(&mut self, index: usize) {
        self.rotate_right(self.inner_length - index);
    }
}

impl<T: Default + Copy, const S: usize> Default for RotatingBuffer<T, S> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::RotatingBuffer;

    #[test]
    fn walk_through() {
        let mut buf = RotatingBuffer::<u8, 32>::new();
        // Original length: 0
        assert_eq!(0, buf.len());
        // Writable buffer size is flexible, but right now: 32
        assert_eq!(32, buf.append_only_mut_slice().len());
        // Let's write! More or less compatible with the way UnixStream::try_read works
        buf.append_only_mut_slice().copy_from_slice(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ]);
        // Amount of bytes read as returned by UnixStream::try_read
        buf.add_len(22);
        // Post read, we rotate right at index 17
        buf.rotate_right_at(17);
        // New length is now 5
        assert_eq!(5, buf.len());
        // Write a full message to the buffer! 27 new spicey bytes
        buf.append_only_mut_slice().copy_from_slice(&[
            22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
            44, 45, 46, 47, 48,
        ]);
        buf.add_len(24);
        assert_eq!(29, buf.len());
        // Rotating again at 21
        buf.rotate_right_at(21);
        assert_eq!(8, buf.len());
        // More new data
        buf.append_only_mut_slice().copy_from_slice(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        // And the new data length
        buf.add_len(16);
        assert_eq!(24, buf.len());
    }
}
