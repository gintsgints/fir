use std::{env::{set_current_dir, self}, path::PathBuf};
use anyhow::Result;

use crate::filelist::{self, read_file_list};

use super::panel_item_context::PanelItemContext;

#[derive(Clone, Default)]
pub struct PanelContext {
    pub active: bool,
    path: PathBuf,
    items: Vec<PanelItemContext>,
    index: usize,
}

impl PanelContext {
    pub fn new(active: bool) -> Result<Self> {
        let path = env::current_dir()?;
        let mut result = PanelContext {
            active,
            path,
            items: vec![],
            index: 0,
        };
        result.read_files()?;
        Ok(result)
    }

    fn current_item(&mut self) -> &mut PanelItemContext {
        self.items
            .get_mut(self.index)
            .expect("Index points on nonexistent file")
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn items(&self) -> Vec<PanelItemContext> {
        self.items.clone()
    }

    pub fn current_item_is_dir(&mut self) -> bool {
        self.current_item().is_dir()
    }

    pub fn current_item_full_path(&mut self) -> String {
        self.current_item().full_path()
    }

    pub fn cd(&mut self) -> Result<()> {
        let file = self.current_filename();
        self.path.push(&file);
        self.path = self.path.canonicalize()?;
        set_current_dir(self.path.display().to_string())?;
        self.read_files()?;
        Ok(())
    }

    pub fn mark_current_item(&mut self) {
        self.current_item().mark()
    }

    pub fn sub_index(&mut self, times: usize) {
        self.index = self.index.saturating_sub(times) % self.items.len();
    }

    pub fn add_index(&mut self, times: usize) {
        let new_index = self.index.saturating_add(times) % self.items.len();
        if new_index > self.index {
            self.index = new_index;
        } else {
            self.index = self.items.len() - 1;
        }
    }

    pub fn path_string(&self) -> String {
        self.path.display().to_string()
    }

    pub fn current_filename(&mut self) -> String {
        filelist::file_name(
            &self
                .items
                .get(self.index)
                .expect("Index points on nonexistent file")
                .path(),
        )
    }

    fn read_files(&mut self) -> Result<&mut Self> {
        self.items = read_file_list(&self.path)?;
        self.items
            .sort_by(|pb_a, pb_b| pb_a.display_string().cmp(&pb_b.display_string()));
        self.items.sort_by_key(|pb| !pb.is_dir());
        self.index = 0;
        Ok(self)
    }
}
