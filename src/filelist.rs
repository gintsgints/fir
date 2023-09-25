use std::{fs, path::PathBuf};

use crate::errors::ProgramError;

pub fn read_file_list(directory: &str) -> Result<Vec<PathBuf>, ProgramError> {
    let paths = fs::read_dir(directory)?;
    let dirs: Vec<PathBuf> = paths.filter_map(|path| Some(path.ok()?.path())).collect();
    Ok(dirs)
}
