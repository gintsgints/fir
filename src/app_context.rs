use std::env;
use std::path::PathBuf;

use crate::{
    commands::Command,
    errors::ProgramError,
    filelist::{self, read_file_list},
};

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
        filelist::file_name(
            self.files
                .get(self.index)
                .expect("Index points on nonexistent file"),
        )
    }

    fn read_files(&mut self) -> Result<(), ProgramError> {
        self.files = read_file_list(&self.path)?;
        self.files
            .sort_by(|pb_a, pb_b| pb_a.display().to_string().cmp(&pb_b.display().to_string()));
        self.files.sort_by_key(|pb| !pb.is_dir());
        self.index = 0;
        Ok(())
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
        let mut left_panel = Panel {
            path: env::current_dir()?,
            files: vec![],
            index: 0,
        };
        left_panel.read_files()?;
        let mut right_panel = Panel {
            path: env::current_dir()?,
            files: vec![],
            index: 0,
        };
        right_panel.read_files()?;

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

    pub fn apply_cmd(&mut self, cmd: Command) -> Result<(), ProgramError> {
        match cmd {
            Command::Cd => {
                let file = self.current_panel().current_filename();
                self.current_panel().path.push(&file);
                self.current_panel().path = self.current_panel().path.canonicalize()?;
                self.current_panel().read_files()?;
            }
        };
        Ok(())
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
