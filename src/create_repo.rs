use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};
pub struct CreateRimg {
    exclude_dir: Option<Vec<String>>,
}

impl CreateRimg {
    pub fn new(exclude: Vec<String>) -> Self {
        Self {
            exclude_dir: Some(exclude),
        }
    }
    fn excluded_path(path: &DirEntry, excluded_path: &Vec<String>) -> bool {
        //if path == path.parent().unwrap().join(path){

        //}
        for paths in excluded_path {
            if path.file_name() == PathBuf::from(paths) {
                return false;
            }
        }
        return true;
    }
    pub fn walk_dir(&self) -> Result<(), Error> {
        let dir = WalkDir::new("./").into_iter();

        if let Some(excluded) = &self.exclude_dir {
            for entry in dir.filter_entry(|f| Self::excluded_path(f, &excluded)) {
                println!("{}", entry?.path().display());
            }
        }
        Ok(())
    }
}
