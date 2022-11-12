use std::fs;
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
        true
    }
    pub fn create_dir() {
        if let Err(err) = fs::create_dir_all(".rimg/refs") {
            println!("Failed to create .rimg directory due to {}", err)
        }
        fs::File::create(".rimg/.rimgtrack").expect("Failed to track file");
    }
    pub fn walk_dir(&self) -> Result<(), Error> {
        let dir = WalkDir::new("./").into_iter();

        if let Some(excluded) = &self.exclude_dir {
            fs::File::create(".rimg/.rimgignore").expect("Unable to create .rimgignore file");
            fs::write(".rimg/.rimgignore", excluded.join("\n")).expect("Failed Operation");
            for entry in dir.filter_entry(|f| Self::excluded_path(f, excluded)) {
                fs::write(
                    "./rimg/.rimgtract",
                    entry?.path().to_str().unwrap().as_bytes(),
                )
                .unwrap();
            }
        }
        Ok(())
    }
}
