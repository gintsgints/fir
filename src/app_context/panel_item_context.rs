use std::path::PathBuf;

#[derive(Clone)]
pub struct PanelItemContext {
    path: PathBuf,
    marked: bool,
}

impl PanelItemContext {
    pub fn new(path: PathBuf) -> Self {
        PanelItemContext {
            path,
            marked: false,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn display_string(&self) -> String {
        self.path.display().to_string()
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn full_path(&self) -> String {
        self.path().display().to_string()
    }

    pub fn marked(&self) -> &bool {
        &self.marked
    }

    pub fn mark(&mut self) {
        self.marked = !self.marked
    }
}
