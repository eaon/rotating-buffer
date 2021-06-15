# Rotating Buffer

… is a small helper data structure that allows a stack-allocated buffer to be
reused while keeping data that couldn't be handled immediately.

Example:

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

Inspired by a pairing session at [Recurse Center](https://www.recurse.com/) 👩‍💻🐙
