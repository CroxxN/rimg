use flate2::{write::DeflateEncoder, Compression};
use std::{
    fs::{self, File},
    io::Write,
};

pub struct Archv;

impl<'a> Archv {
    fn hash(msg: &'a str) {
        todo!()
    }
    pub fn flate() {
        let mut archive = DeflateEncoder::new(
            File::create("./.rimg/refObj").unwrap(),
            Compression::default(),
        );
        let file = fs::read("Cargo.toml").unwrap();
        archive.write_all(&file).unwrap();
        println!("{:?}", archive.finish().unwrap());
    }
}
