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
        fs::write(".rimg/.ringtrack", path.join("\n")).expect("Operation Failed");
    }
    // pub fn list_dir_names(&self) {
    //     println!("{}", self);
    // }
}
