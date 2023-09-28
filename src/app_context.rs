use std::path::PathBuf;

use crate::{errors::ProgramError, filelist::read_file_list};

#[derive(Clone, PartialEq)]
enum SelectedPanel {
    Left,
    Right,
}

#[derive(Clone)]
pub struct AppContext {
    pub should_quit: bool,
    left_panel_path: String,
    right_panel_path: String,
    left_files_list: Vec<PathBuf>,
    right_files_list: Vec<PathBuf>,
    current_panel: SelectedPanel,
    left_selection_index: usize,
    right_selection_index: usize,
}

impl AppContext {
    pub fn new(left_path: &str, right_path: &str) -> Result<Self, ProgramError> {
        let mut files_left = read_file_list(left_path)?;
        files_left
            .sort_by(|pb_a, pb_b| pb_a.display().to_string().cmp(&pb_b.display().to_string()));
        files_left.sort_by_key(|pb| !pb.is_dir());
        let mut files_right = read_file_list(right_path)?;
        files_right
            .sort_by(|pb_a, pb_b| pb_a.display().to_string().cmp(&pb_b.display().to_string()));
        files_right.sort_by_key(|pb| !pb.is_dir());

        Ok(AppContext {
            should_quit: false,
            left_panel_path: left_path.to_string(),
            right_panel_path: right_path.to_string(),
            left_files_list: files_left,
            right_files_list: files_right,
            current_panel: SelectedPanel::Left,
            right_selection_index: 0,
            left_selection_index: 0,
        })
    }

    pub fn tab(&mut self) {
        match self.current_panel {
            SelectedPanel::Left => {
                self.current_panel = SelectedPanel::Right;
            }
            SelectedPanel::Right => {
                self.current_panel = SelectedPanel::Left;
            }
        }
    }

    pub fn key_up(&mut self) {
        match self.current_panel {
            SelectedPanel::Left => {
                self.left_selection_index =
                    self.left_selection_index.saturating_sub(1) % self.left_files_list.len();
            }
            SelectedPanel::Right => {
                self.right_selection_index =
                    self.right_selection_index.saturating_sub(1) % self.right_files_list.len();
            }
        }
    }

    pub fn key_down(&mut self) {
        match self.current_panel {
            SelectedPanel::Left => {
                self.left_selection_index =
                    self.left_selection_index.saturating_add(1) % self.left_files_list.len();
            }
            SelectedPanel::Right => {
                self.right_selection_index =
                    self.right_selection_index.saturating_add(1) % self.right_files_list.len();
            }
        }
    }

    pub fn right_selection_index(&self) -> Option<usize> {
        match self.current_panel {
            SelectedPanel::Right => Some(self.right_selection_index),
            SelectedPanel::Left => None,
        }
    }

    pub fn left_selection_index(&self) -> Option<usize> {
        match self.current_panel {
            SelectedPanel::Left => Some(self.left_selection_index),
            SelectedPanel::Right => None,
        }
    }

    pub fn is_right_active(&self) -> bool {
        self.current_panel == SelectedPanel::Right
    }

    pub fn is_left_active(&self) -> bool {
        self.current_panel == SelectedPanel::Left
    }

    pub fn right_path(&self) -> String {
        self.right_panel_path.clone()
    }

    pub fn left_path(&self) -> String {
        self.left_panel_path.clone()
    }

    pub fn left_files(&self) -> Vec<PathBuf> {
        self.left_files_list.clone()
    }

    pub fn right_files(&self) -> Vec<PathBuf> {
        self.right_files_list.clone()
    }
}
