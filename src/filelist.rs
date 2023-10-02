use std::{fs, path::PathBuf};

use crate::errors::ProgramError;

pub fn read_file_list(directory: &str) -> Result<Vec<PathBuf>, ProgramError> {
    let paths = fs::read_dir(directory)?;
    let dirs: Vec<PathBuf> = paths.filter_map(|path| Some(path.ok()?.path())).collect();
    Ok(dirs)
}

pub fn file_name(fb: &PathBuf) -> String {
    String::from(fb.file_name().expect("Cannot convert file name").to_str().expect("Not able to convert file name to string"))
}