use std::path::{Path, PathBuf};

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};

#[derive(Debug, Clone)]
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

    pub fn file_name(&self, fb: &Path) -> String {
        match fb.file_name() {
            Some(found_name) => String::from(
                found_name
                    .to_str()
                    .expect("Not able to convert file name to string"),
            ),
            None => String::from(".."),
        }
    }

    pub fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    pub fn display_string(&self) -> String {
        self.path.display().to_string()
    }

    pub fn file_full_path(&self) -> String {
        self.path.display().to_string()
    }

    pub fn current_file_name(&self) -> String {
        self.file_name(&self.path)
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn marked(&self) -> &bool {
        &self.marked
    }
}

impl From<&PanelItem> for ListItem<'_> {
    fn from(val: &PanelItem) -> Self {
        let style = if *val.marked() {
            Style::new().fg(Color::Yellow)
        } else if val.path().is_dir() {
            Style::new().fg(Color::White)
        } else {
            Style::new().fg(Color::Cyan)
        };
        let file_name = val.file_name(val.path());
        let file_line = Line::from(vec![Span::styled(file_name.to_string(), style)]);
        ListItem::new(vec![file_line])
    }
}
