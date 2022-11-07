use std::io::Write;

use flate2::{write::DeflateEncoder, Compression};
pub(crate) fn flate() {
    let mut archive = DeflateEncoder::new(Vec::new(), Compression::default());
    archive.write_all(b"Hello world").unwrap();
    println!("{:?}", archive.finish().unwrap());
}
