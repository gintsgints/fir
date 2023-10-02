use std::env;
use std::path::PathBuf;

use crate::{commands::Command, errors::ProgramError, filelist::{read_file_list, self}};

#[derive(Clone, PartialEq)]
enum SelectedPanel {
    Left,
    Right,
}

#[derive(Clone)]
pub struct Panel {
    path: PathBuf,
    files: Vec<PathBuf>,
    index: usize,
}

impl Panel {
    fn current_filename(&mut self) -> String {
        filelist::file_name(self.files.get(self.index).expect("Index points on nonexistent file"))
    }
}

#[derive(Clone)]
pub struct AppContext {
    pub should_quit: bool,
    left_panel: Panel,
    right_panel: Panel,
    current_panel: SelectedPanel,
}

impl AppContext {
    pub fn new() -> Result<Self, ProgramError> {
        let left_path = env::current_dir()?;
        let right_path = env::current_dir()?;
        let mut files_left = read_file_list(&left_path)?;
        files_left
            .sort_by(|pb_a, pb_b| pb_a.display().to_string().cmp(&pb_b.display().to_string()));
        files_left.sort_by_key(|pb| !pb.is_dir());
        let mut files_right = read_file_list(&right_path)?;
        files_right
            .sort_by(|pb_a, pb_b| pb_a.display().to_string().cmp(&pb_b.display().to_string()));
        files_right.sort_by_key(|pb| !pb.is_dir());

        let left_panel = Panel {
            path: left_path,
            files: files_left,
            index: 0,
        };
        let right_panel = Panel {
            path: right_path,
            files: files_right,
            index: 0,
        };

        Ok(AppContext {
            should_quit: false,
            left_panel,
            right_panel,
            current_panel: SelectedPanel::Left,
        })
    }

    fn current_panel(&mut self) -> &mut Panel {
        match self.current_panel {
            SelectedPanel::Left => &mut self.left_panel,
            SelectedPanel::Right => &mut self.right_panel,
        }
    }

    pub fn apply_cmd(&mut self, cmd: Command) {
        match cmd {
            Command::cd => {
                // self.current_path()
                // self.current_panel().path = self.current_panel().current_filename();
                // self.current_panel().path.push("path")?;
            }
        }
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
                self.left_panel.index =
                    self.left_panel.index.saturating_sub(1) % self.left_panel.files.len();
            }
            SelectedPanel::Right => {
                self.right_panel.index =
                    self.right_panel.index.saturating_sub(1) % self.right_panel.files.len();
            }
        }
    }

    pub fn key_down(&mut self) {
        match self.current_panel {
            SelectedPanel::Left => {
                self.left_panel.index =
                    self.left_panel.index.saturating_add(1) % self.left_panel.files.len();
            }
            SelectedPanel::Right => {
                self.right_panel.index =
                    self.right_panel.index.saturating_add(1) % self.right_panel.files.len();
            }
        }
    }

    pub fn right_selection_index(&self) -> Option<usize> {
        match self.current_panel {
            SelectedPanel::Right => Some(self.right_panel.index),
            SelectedPanel::Left => None,
        }
    }

    pub fn left_selection_index(&self) -> Option<usize> {
        match self.current_panel {
            SelectedPanel::Left => Some(self.left_panel.index),
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
        self.right_panel.path.display().to_string()
    }

    pub fn left_path(&self) -> String {
        self.left_panel.path.display().to_string()
    }

    pub fn left_files(&self) -> Vec<PathBuf> {
        self.left_panel.files.clone()
    }

    pub fn right_files(&self) -> Vec<PathBuf> {
        self.right_panel.files.clone()
    }
}
