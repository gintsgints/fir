use std::env::{self, set_current_dir};
use std::path::PathBuf;
use std::process::Command;

use crate::{
    commands::AppCommand,
    errors::ProgramError,
    filelist::{self, read_file_list},
};

#[derive(Clone, PartialEq)]
enum ActivePanel {
    Left,
    Right,
}

#[derive(Clone)]
pub struct PanelItem {
    path: PathBuf,
    marked: bool,
}

impl PanelItem {
    pub fn new(path: PathBuf) -> Self {
        PanelItem {
            path,
            marked: false,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn marked(&self) -> &bool {
        &self.marked
    }

    pub fn mark(&mut self) {
        self.marked = !self.marked
    }
}

#[derive(Clone)]
pub struct Panel {
    path: PathBuf,
    items: Vec<PanelItem>,
    index: usize,
}

impl Panel {
    fn current_item(&mut self) -> &mut PanelItem {
        self.items
            .get_mut(self.index)
            .expect("Index points on nonexistent file")
    }

    fn current_filename(&mut self) -> String {
        filelist::file_name(
            &self
                .items
                .get(self.index)
                .expect("Index points on nonexistent file")
                .path,
        )
    }

    fn read_files(&mut self) -> Result<(), ProgramError> {
        self.items = read_file_list(&self.path)?;
        self.items.sort_by(|pb_a, pb_b| {
            pb_a.path
                .display()
                .to_string()
                .cmp(&pb_b.path.display().to_string())
        });
        self.items.sort_by_key(|pb| !pb.path.is_dir());
        self.index = 0;
        Ok(())
    }
}

#[derive(Clone)]
pub struct AppContext {
    pub should_quit: bool,
    left_panel: Panel,
    right_panel: Panel,
    active_panel: ActivePanel,
}

impl AppContext {
    pub fn new() -> Result<Self, ProgramError> {
        let mut left_panel = Panel {
            path: env::current_dir()?,
            items: vec![],
            index: 0,
        };
        left_panel.read_files()?;
        let mut right_panel = Panel {
            path: env::current_dir()?,
            items: vec![],
            index: 0,
        };
        right_panel.read_files()?;

        Ok(AppContext {
            should_quit: false,
            left_panel,
            right_panel,
            active_panel: ActivePanel::Left,
        })
    }

    pub fn current_panel(&mut self) -> &mut Panel {
        match self.active_panel {
            ActivePanel::Left => &mut self.left_panel,
            ActivePanel::Right => &mut self.right_panel,
        }
    }

    pub fn current_item_is_dir(&mut self) -> bool {
        self.current_panel().current_item().path.is_dir()
    }

    pub fn current_item_full_path(&mut self) -> String {
        self.current_panel()
            .current_item()
            .path()
            .display()
            .to_string()
    }

    pub fn apply_cmd(&mut self, cmd: AppCommand) -> Result<(), ProgramError> {
        match cmd {
            AppCommand::Cd => {
                let file = self.current_panel().current_filename();
                self.current_panel().path.push(&file);
                self.current_panel().path = self.current_panel().path.canonicalize()?;
                set_current_dir(self.current_panel().path.display().to_string())?;
                self.current_panel().read_files()?;
            }
            AppCommand::Open => {
                #[cfg(target_os = "windows")]
                Command::new("start")
                    .arg(&self.current_file().display().to_string())
                    .spawn()
                    .expect("Failed to open file");
                #[cfg(not(target_os = "windows"))]
                Command::new("open")
                    .arg(&self.current_item_full_path())
                    .spawn()
                    .expect("Failed to open file");
            }
        };
        Ok(())
    }

    pub fn tab(&mut self) {
        match self.active_panel {
            ActivePanel::Left => {
                self.active_panel = ActivePanel::Right;
            }
            ActivePanel::Right => {
                self.active_panel = ActivePanel::Left;
            }
        }
    }

    pub fn key_up(&mut self, times: usize, with_select: bool) {
        if with_select {
            self.current_panel().current_item().mark();
        }
        self.current_panel().index =
            self.current_panel().index.saturating_sub(times) % self.current_panel().items.len();
    }

    pub fn key_down(&mut self, times: usize, with_select: bool) {
        if with_select {
            self.current_panel().current_item().mark();
        }
        let new_index =
            self.current_panel().index.saturating_add(times) % self.current_panel().items.len();
        if new_index > self.current_panel().index {
            self.current_panel().index = new_index;
        } else {
            self.current_panel().index = self.current_panel().items.len() - 1;
        }
    }

    pub fn right_selection_index(&self) -> Option<usize> {
        match self.active_panel {
            ActivePanel::Right => Some(self.right_panel.index),
            ActivePanel::Left => None,
        }
    }

    pub fn left_selection_index(&self) -> Option<usize> {
        match self.active_panel {
            ActivePanel::Left => Some(self.left_panel.index),
            ActivePanel::Right => None,
        }
    }

    pub fn right_path(&self) -> String {
        self.right_panel.path.display().to_string()
    }

    pub fn left_path(&self) -> String {
        self.left_panel.path.display().to_string()
    }

    pub fn left_files(&self) -> Vec<PanelItem> {
        self.left_panel.items.clone()
    }

    pub fn right_files(&self) -> Vec<PanelItem> {
        self.right_panel.items.clone()
    }
}
