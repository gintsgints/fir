pub mod panel_context;
pub mod panel_item_context;

use std::env::{self};
use std::process::Command;

use crate::{commands::AppCommand, errors::ProgramError};

use self::panel_context::PanelContext;
use self::panel_item_context::PanelItemContext;

#[derive(Clone, PartialEq)]
enum ActivePanel {
    Left,
    Right,
}

#[derive(Clone)]
pub struct AppContext {
    pub should_quit: bool,
    left_panel: PanelContext,
    right_panel: PanelContext,
    active_panel: ActivePanel,
}

impl AppContext {
    pub fn new() -> Result<Self, ProgramError> {
        Ok(AppContext {
            should_quit: false,
            left_panel: PanelContext::new(env::current_dir()?)?.to_owned(),
            right_panel: PanelContext::new(env::current_dir()?)?.to_owned(),
            active_panel: ActivePanel::Left,
        })
    }

    pub fn current_panel(&mut self) -> &mut PanelContext {
        match self.active_panel {
            ActivePanel::Left => &mut self.left_panel,
            ActivePanel::Right => &mut self.right_panel,
        }
    }

    pub fn current_item_is_dir(&mut self) -> bool {
        self.current_panel().current_item_is_dir()
    }

    pub fn current_item_full_path(&mut self) -> String {
        self.current_panel().current_item_full_path()
    }

    pub fn apply_cmd(&mut self, cmd: AppCommand) -> Result<(), ProgramError> {
        match cmd {
            AppCommand::Cd => self.current_panel().cd()?,
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
            self.current_panel().mark_current_item();
        }
        self.current_panel().sub_index(times);
    }

    pub fn key_down(&mut self, times: usize, with_select: bool) {
        if with_select {
            self.current_panel().mark_current_item();
        }
        self.current_panel().add_index(times);
    }

    pub fn right_selection_index(&self) -> Option<usize> {
        match self.active_panel {
            ActivePanel::Right => Some(self.right_panel.index()),
            ActivePanel::Left => None,
        }
    }

    pub fn left_selection_index(&self) -> Option<usize> {
        match self.active_panel {
            ActivePanel::Left => Some(self.left_panel.index()),
            ActivePanel::Right => None,
        }
    }

    pub fn right_path(&self) -> String {
        self.right_panel.path_string()
    }

    pub fn left_path(&self) -> String {
        self.left_panel.path_string()
    }

    pub fn left_files(&self) -> Vec<PanelItemContext> {
        self.left_panel.items()
    }

    pub fn right_files(&self) -> Vec<PanelItemContext> {
        self.right_panel.items()
    }
}
