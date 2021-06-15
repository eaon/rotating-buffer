use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

use rotating_buffer::RotatingBuffer;

fn main() -> std::io::Result<()> {
    // Unusually small, but we're just proving a point here. Let's just assume we have incredibly
    // limited bandwidth and flexible value lengths, separated by a comma
    let mut buf = RotatingBuffer::<u8, 5>::new();

    let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    loop {
        let read_size = stream.read(buf.get_append_only())?;
        if read_size != 0 {
            // read_size and _buf_size may diverge because add_len extends data that was
            // right-rotated previously
            let _buf_size = buf.add_len(read_size);
        }

        // .as_slice() will provide a slice combining previous right-rotated
        // data as well as newly written data
        let incoming = str::from_utf8(buf.as_slice()).unwrap();

        // Print every comma separated value on a separate line, even if the buffer is too small to
        // hold the entire value, except if it's "EOF\n"
        if !incoming.ends_with("EOF\n") && read_size != 0 || incoming.len() > 4 {
            if let Some(index) = incoming.rfind(',') {
                for value in incoming[..index].split(',') {
                    println!("{}", value);
                }
                // Do not include the comma when rotating and resizing
                buf.rotate_right_and_resize_at(index + 1);
            } else {
                // Here we could push to a heap-allocated structure if appropriate, but we'd need to
                // adapt the logic above to account for when this value is completed by a comma
                print!("{}", incoming);
                buf.resize(0);
            }
        } else {
            return Ok(());
        }
    }
}
