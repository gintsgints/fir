use std::path::PathBuf;

use crate::{filelist::read_file_list, errors::ProgramError};

#[derive(Debug, Default, Clone)]
pub struct AppContext {
    left_panel_path: String,
    right_panel_path: String,
    left_files: Vec<PathBuf>,
    right_files: Vec<PathBuf>,
}

impl AppContext {
    pub fn new(left_path: &str, right_path: &str) -> Result<Self, ProgramError> {
        let fl = read_file_list(left_path)?;
        let fr = read_file_list(right_path)?;
        Ok(AppContext {
            left_panel_path: left_path.to_string(),
            right_panel_path: right_path.to_string(),
            left_files: fl,
            right_files: fr,
        })
    }

    pub fn right_path(&self) -> String {
        self.right_panel_path.clone()
    }

    pub fn left_path(&self) -> String {
        self.left_panel_path.clone()
    }
}
