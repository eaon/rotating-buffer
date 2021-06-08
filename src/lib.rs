pub struct OverflowingBuffer<T: Default + Copy, const S: usize> {
    inner_length: usize,
    inner: [T; S],
    overflow: usize,
    overflow_length: usize,
}

#[macro_export]
macro_rules! obuf {
    ($type:ident, $size:expr, $overflow:expr) => {
        OverflowingBuffer {
            inner: [$type::default(); $size + $overflow],
            inner_length: 0,
            overflow: $overflow,
            overflow_length: 0,
        }
    };
}

impl<T: Default + Copy, const S: usize> OverflowingBuffer<T, S> {
    pub fn as_slice(&self) -> &[T] {
        &self.inner[..self.inner_length]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.inner[self.overflow_length..(S - self.overflow) + self.overflow_length]
    }

    pub fn is_empty(&self) -> bool {
        self.inner_length == 0
    }

    pub fn len(&self) -> usize {
        self.inner_length
    }

    pub fn set_len(&mut self, length: usize) {
        if length > S - self.overflow {
            panic!(
                "Input length ({}) is larger than internal buffer capacity ({})",
                length,
                S - self.overflow
            );
        }

        self.inner_length = self.overflow_length + length;
        self.overflow_length = 0;
    }

    pub fn overflow_at(&mut self, index: usize) {
        if (self.inner_length - index) > self.overflow {
            panic!(
                "Overflow buffer too small to allow a full write: {} > {} / {} > {}",
                (self.inner_length - index) + (S - self.overflow),
                S - self.overflow,
                (self.inner_length - index),
                self.overflow,
            );
        }

        let mut new = [T::default(); S];
        self.overflow_length = self.inner[index..self.inner_length].len();
        new[..self.overflow_length].copy_from_slice(&self.inner[index..self.inner_length]);
        self.inner = new;
        self.inner_length -= index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> OverflowingBuffer<u8, 32> {
        let mut whut = obuf!(u8, 24, 8);
        // Original length: 0
        assert_eq!(0, whut.len());
        // Writable buffer size: 24
        assert_eq!(24, whut.as_mut_slice().len());
        // Let's write! More or less compatible with the way UnixStream::try_read works
        whut.as_mut_slice().copy_from_slice(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 0, 0,
        ]);
        // Amount of bytes read as returned by UnixStream::try_read
        whut.set_len(22);
        println!("Buffer contents: {:?}", whut.as_slice());
        // Post read, we overflow at index 17
        whut.overflow_at(17);
        // New length
        assert_eq!(5, whut.len());
        // Write a full message to the buffer! 24 new spicey bytes
        whut.as_mut_slice().copy_from_slice(&[
            22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
            44, 45,
        ]);
        whut.set_len(24);
        println!(
            "Wrote 24 new bytes to buffer\nNew buffer contents: {:?}",
            whut.as_slice()
        );
        assert_eq!(29, whut.len());
        // Overflow again at 21
        whut.overflow_at(21);
        assert_eq!(8, whut.len());
        // More new data
        whut.as_mut_slice().copy_from_slice(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 0, 0, 0, 0, 0, 0, 0,
        ]);
        // And the data length
        whut.set_len(16);
        println!(
            "Wrote 16 new bytes to buffer\nNew buffer contents: {:?}",
            whut.as_slice()
        );
        assert_eq!(24, whut.len());
        whut
    }

    #[test]
    fn walk_through() {
        base();
    }

    #[test]
    #[should_panic]
    fn walk_through_overflow_the_overflow_buffer() {
        let mut whut = base();
        println!("Overflowing at index 4, will panic:");
        // will panic
        whut.overflow_at(4);
    }
}
