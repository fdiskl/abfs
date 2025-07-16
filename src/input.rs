use std::io::{self, Read};

use crate::graph::Graph;

pub fn read_graph<R: Read>(r: &mut R) -> io::Result<(Graph, usize)> {
    let n: usize = read_number(r)?;

    let mut vals = vec![];
    vals.reserve(n * n);

    for _ in 0..n {
        for _ in 0..n {
            vals.push(read_number(r)?);
        }
    }

    Ok((
        Graph {
            values: vals.into(),
        },
        n,
    ))
}

fn read_number<R: Read, T: std::str::FromStr>(reader: &mut R) -> io::Result<T>
where
    T::Err: std::fmt::Debug,
{
    let mut buf = Vec::new();
    let mut byte = [0u8; 1];

    while reader.read(&mut byte)? == 1 {
        let b = byte[0];
        if b == b',' || b.is_ascii_whitespace() {
            if !buf.is_empty() {
                break;
            }
            continue;
        }
        buf.push(b);
    }

    let s = String::from_utf8(buf).expect("Invalid UTF-8");
    let num = s.parse::<T>().expect("Parse failed");
    Ok(num)
}
