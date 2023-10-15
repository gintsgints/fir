use std::{fs, path::PathBuf};
use anyhow::Result;

use crate::app_context::panel_item_context::PanelItemContext;

pub fn read_file_list(directory: &PathBuf) -> Result<Vec<PanelItemContext>> {
    let paths = fs::read_dir(directory)?;
    let mut dirs: Vec<PanelItemContext> = paths
        .filter_map(|path| Some(PanelItemContext::new(path.ok()?.path())))
        .collect();
    let mut back_path = PathBuf::new();
    back_path.push("..");
    dirs.push(PanelItemContext::new(back_path));
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
