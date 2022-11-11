use walkdir::WalkDir;

use crate::archive::CreateArcv;

pub fn save() {
    WalkDir::new("./")
        .into_iter()
        .for_each(|dirs| dirs.unwrap().create_arcv());
    // File::open("./rimg/.rimgtrack")
    //         .unwrap()
    //         .write_all(&mut arc_files);
    //     arc_files.lines().into_iter().for_each(|line|
    //         line.unwrap().
    //         )
    // Archv::flate()
}
