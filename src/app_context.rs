pub mod editor_context;
pub mod panel_context;
pub mod panel_item_context;

use std::process::Command;

use tui_textarea::Input;

use crate::{commands::AppCommand, errors::ProgramError};

use self::editor_context::EditorContext;
use self::panel_context::PanelContext;
use self::panel_item_context::PanelItemContext;

#[derive(Clone)]
pub struct AppContext<'a> {
    pub should_quit: bool,
    left_context: PanelContext,
    right_context: PanelContext,
    editor_context: EditorContext<'a>,
}

impl<'a> AppContext<'a> {
    pub fn new() -> Result<Self, ProgramError> {
        Ok(AppContext {
            should_quit: false,
            left_context: PanelContext::new(true)?.to_owned(),
            right_context: PanelContext::new(false)?.to_owned(),
            editor_context: EditorContext::new().to_owned(),
        })
    }

    pub fn current_panel(&mut self) -> &mut PanelContext {
        if self.left_context.active {
            &mut self.left_context
        } else {
            &mut self.right_context
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
            AppCommand::Edit => {
                let path = self.current_panel().current_item_full_path();
                self.editor_context.open(path)?
            }
            AppCommand::Open => {
                #[cfg(target_os = "windows")]
                Command::new("start")
                    .arg(&self.current_item_full_path())
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
        self.left_context.active = !self.left_context.active;
        self.right_context.active = !self.right_context.active;
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

    pub fn close_editor(&mut self) {
        self.editor_context.close();
    }

    pub fn editor_update(&mut self) -> Result<(), ProgramError> {
        self.editor_context.update()?;
        Ok(())
    }

    pub fn editor_context(&self) -> &EditorContext {
        &self.editor_context
    }

    pub fn right_context(&self) -> &PanelContext {
        &self.right_context
    }

    pub fn left_context(&self) -> &PanelContext {
        &self.left_context
    }

    pub fn right_path(&self) -> String {
        self.right_context.path_string()
    }

    pub fn left_path(&self) -> String {
        self.left_context.path_string()
    }

    pub fn left_files(&self) -> Vec<PanelItemContext> {
        self.left_context.items()
    }

    pub fn right_files(&self) -> Vec<PanelItemContext> {
        self.right_context.items()
    }
}
