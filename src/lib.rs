#![no_std]

/*!
# Rotating Buffer

… is a small helper data structure that allows a stack-allocated buffer to be
reused while keeping data that couldn't be handled immediately.

## Example

```rust
use rotating_buffer::*;
let mut buf = RotatingBuffer::<u8, 4>::new();

buf.get_append_only().copy_from_slice(&[1, 2, 3, 4]);
buf.add_len(4);
assert_eq!(&[1, 2, 3, 4], buf.as_slice());

buf.rotate_right_and_resize_at(3);
assert_eq!(&[4], buf.as_slice());

assert_eq!(3, buf.get_append_only().len());
buf.get_append_only().copy_from_slice(&[5, 6, 7]);
buf.add_len(3);
assert_eq!(&[4, 5, 6, 7], buf.as_slice());

buf.rotate_right_and_resize_at(4);
assert_eq!(buf.as_slice().len(), 0);
```

For a more in depth example please see `examples/read_to_eof.rs`.

Inspired by a pairing session at [Recurse Center](https://www.recurse.com/) 👩‍💻🐙
*/

#[derive(Debug, Clone, Copy)]
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

    /// Extracts slice with the length of the buffer's internally tracked size
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rotating_buffer::*;
    /// let mut buf = RotatingBuffer::<u128, 20>::new();
    /// assert_eq!(buf.as_slice().len(), 0);
    ///
    /// buf.add_len(15);
    /// assert_eq!(buf.as_slice().len(), 15);
    /// ```
    pub fn as_slice(&self) -> &[T] {
        &self.inner[..self.inner_length]
    }

    /// Returns a mutable slice with the length of the currently "unused" allocation of the buffer
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rotating_buffer::*;
    /// let mut buf = RotatingBuffer::<u8, 5>::new();
    /// buf.add_len(3);
    ///
    /// assert_eq!(buf.get_append_only().len(), 2);
    /// buf.get_append_only()[0] = 50;
    /// assert_eq!(buf.inner, [0, 0, 0, 50, 0]);
    /// ```
    pub fn get_append_only(&mut self) -> &mut [T] {
        &mut self.inner[self.inner_length..]
    }

    /// Returns `true` if the buffer's internal size is 0
    pub fn is_empty(&self) -> bool {
        self.inner_length == 0
    }

    /// Returns internally tracked length
    pub fn len(&self) -> usize {
        self.inner_length
    }

    /// Returns the capacity of this buffer
    pub fn capacity(&self) -> usize {
        S
    }

    /// Manually set the internal size of the buffer
    ///
    /// # Panics
    ///
    /// Panics if the new size is bigger than its capacity
    pub fn resize(&mut self, new_len: usize) {
        assert!(new_len <= S);
        self.inner_length = new_len;
    }

    /// Add to the current internal length of the buffer
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rotating_buffer::*;
    /// let mut buf = RotatingBuffer::<u8, 5>::new();
    /// assert_eq!(buf.len(), 0);
    ///
    /// buf.add_len(3);
    /// assert_eq!(buf.len(), 3);
    /// buf.add_len(1);
    /// assert_eq!(buf.len(), 4);
    /// ```
    pub fn add_len(&mut self, new_len: usize) -> usize {
        self.resize(self.inner_length + new_len);
        self.inner_length
    }

    /// Rotates the buffer contents in place (see `core::slice::rotate_right`) and subsequently
    /// changes the buffer's internal length to however much was rotated
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rotating_buffer::*;
    /// let mut buf = RotatingBuffer::<bool, 5>::new();
    /// buf.get_append_only()[3] = true;
    /// buf.add_len(5);
    ///
    /// buf.rotate_right_and_resize(2);
    /// assert_eq!(buf.as_slice()[0], true);
    /// assert_eq!(buf.len(), 2);
    /// ```
    pub fn rotate_right_and_resize(&mut self, k: usize) {
        self.inner[..self.inner_length].rotate_right(k);
        self.inner_length = k;
    }

    /// Rotate and resize buffer by supplying an index rather than a length
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rotating_buffer::*;
    /// let mut buf = RotatingBuffer::<bool, 5>::new();
    /// buf.get_append_only()[3] = true;
    /// buf.add_len(5);
    ///
    /// buf.rotate_right_and_resize_at(3);
    /// assert_eq!(buf.as_slice()[0], true);
    /// assert_eq!(buf.len(), 2);
    /// ```
    pub fn rotate_right_and_resize_at(&mut self, index: usize) {
        self.rotate_right_and_resize(self.inner_length - index);
    }
}

/// Maybe just to allow `RotatingBuffer<RotatingBuffer<T, S>, S>` 😄
///
/// # Example
///
/// ```rust
/// # use rotating_buffer::*;
/// let mut buf = RotatingBuffer::<RotatingBuffer<u8, 10>, 5>::new();
/// buf.add_len(2);
/// let slice = buf.as_slice();
/// assert_eq!(slice[0].inner, slice[1].inner);
/// ```
///
/// But why!
impl<T: Default + Copy, const S: usize> Default for RotatingBuffer<T, S> {
    fn default() -> Self {
        Self::new()
    }
}
