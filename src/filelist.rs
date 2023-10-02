use std::{fs, path::PathBuf};

use crate::errors::ProgramError;

pub fn read_file_list(directory: &PathBuf) -> Result<Vec<PathBuf>, ProgramError> {
    let paths = fs::read_dir(directory)?;
    let mut dirs: Vec<PathBuf> = paths.filter_map(|path| Some(path.ok()?.path())).collect();
    let mut back_path = PathBuf::new();
    back_path.push("..");
    dirs.push(back_path);
    Ok(dirs)
}

pub fn file_name(fb: &PathBuf) -> String {
    match fb.file_name() {
        Some(found_name) => String::from(
            found_name
                .to_str()
                .expect("Not able to convert file name to string"),
        ),
        None => String::from(".."),
    }
}
