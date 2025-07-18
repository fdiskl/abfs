use fast_float::FastFloat;
use std::io::{self, BufReader, Read};

use crate::graph::Graph;

pub fn read_graph<R: Read>(reader: &mut R) -> io::Result<(Graph, usize)> {
    let mut buf = Vec::new();
    let mut br = BufReader::new(reader);
    br.read_to_end(&mut buf)?;

    let mut pos = 0;

    // Parse first number = n (usize, integer)
    let (n, new_pos) = parse_number_usize(&buf, pos)?;
    pos = new_pos;
    let n = n as usize;

    let mut vals = Vec::with_capacity(n * n);

    for i in 0..n {
        if i % 1000 == 0 {
            println!("Reading row {}", i);
        }
        for _ in 0..n {
            let (val, new_pos) = parse_number_f32(&buf, pos)?;
            pos = new_pos;
            vals.push(val);
        }
    }

    Ok((
        Graph {
            values: vals.into(),
        },
        n,
    ))
}

fn parse_number_usize(buf: &[u8], mut pos: usize) -> io::Result<(usize, usize)> {
    while pos < buf.len() && (buf[pos] == b',' || buf[pos].is_ascii_whitespace()) {
        pos += 1;
    }
    if pos >= buf.len() {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "EOF while parsing usize",
        ));
    }

    let start = pos;
    while pos < buf.len() && buf[pos].is_ascii_digit() {
        pos += 1;
    }

    if start == pos {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected digit"));
    }

    let s = std::str::from_utf8(&buf[start..pos])
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
    let val = s
        .parse::<usize>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse usize"))?;
    Ok((val, pos))
}

fn parse_number_f32(buf: &[u8], mut pos: usize) -> io::Result<(f32, usize)> {
    while pos < buf.len() && (buf[pos] == b',' || buf[pos].is_ascii_whitespace()) {
        pos += 1;
    }
    if pos >= buf.len() {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "EOF while parsing f32",
        ));
    }

    let start = pos;
    // float chars: digits, '.', 'e', 'E', '+', '-'
    while pos < buf.len()
        && (buf[pos].is_ascii_digit()
            || buf[pos] == b'.'
            || buf[pos] == b'e'
            || buf[pos] == b'E'
            || buf[pos] == b'+'
            || buf[pos] == b'-')
    {
        pos += 1;
    }

    if start == pos {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected float"));
    }

    let slice = &buf[start..pos];

    // Use fast_float to parse float from bytes
    match fast_float::parse::<f32, &[u8]>(slice) {
        Ok(val) => Ok((val, pos)),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Failed to parse float",
        )),
    }
}
