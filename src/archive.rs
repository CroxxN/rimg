// use flate2::{write::DeflateEncoder, Compression};
// use std::fs::File;
use walkdir::DirEntry;

// pub struct Archv;

// impl<'a> Archv {
//     fn hash(msg: &'a str) {
//         todo!()
//     }
//     pub fn flater(dir: &'a str) {
//         let mut archive = DeflateEncoder::new(
//             File::create("./.rimg/refObj").unwrap(),
//             Compression::default(),
//         );
//     }
// }

pub trait CreateArcv {
    fn create_arcv(self);
}
impl CreateArcv for DirEntry {
    fn create_arcv(self) {
        println!("{:?}", self.path().display());
    }
}
