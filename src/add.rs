use std::io::Write;
use std::{fmt::Display, fs};

pub struct Add<'a> {
    path: &'a Vec<String>,
}

impl Display for Add<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path)?;
        Ok(())
    }
}

impl<'a> Add<'a> {
    pub fn new(path: &'a Vec<String>) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(".rimg/.rimgtrack")
            .unwrap();
        let mut binding = path.join("\n");
        binding.insert_str(0, "\n");
        let contents = binding.as_bytes();

        if let Err(e) = file.write(contents) {
            eprintln!("{}", e);
        }
    }
    // pub fn list_dir_names(&self) {
    //     println!("{}", self);
    // }
}
