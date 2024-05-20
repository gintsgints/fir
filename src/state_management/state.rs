use std::{
    env,
    fs::read_dir,
    path::{Path, PathBuf},
};

use anyhow::Ok;

use super::{action::Action, panel_item::PanelItem, state_store::PopupType, PanelPosition};

#[derive(Debug, Clone, Default)]
pub struct PanelData {
    pub active: bool,
    pub path: PathBuf,
    pub index: usize,
    pub items: Vec<PanelItem>,
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub l_panel: PanelData,
    pub r_panel: PanelData,
    /// Editor state
    /// If None, editors page is not rendered. If there is value, it points to file name to be edited.
    pub editor_file: Option<String>,
    pub editor_modified: bool,
    /// App state
    /// Popup
    pub popup_msg: String,
    pub popup_type: PopupType,
    pub popup_next_action: Option<Action>,
    pub popup_input: String,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let path = env::current_dir()?;
        let items = read_items(&path);
        let state = State {
            l_panel: PanelData {
                active: true,
                path: path.clone(),
                index: 0,
                items: items.clone(),
            },
            r_panel: PanelData {
                active: false,
                path,
                index: 0,
                items,
            },
            ..Default::default()
        };
        Ok(state)
    }

    fn active_panel(&self) -> &PanelData {
        if self.l_panel.active {
            return &self.l_panel;
        }
        &self.r_panel
    }

    fn set_active_panel_index(&mut self, index: usize) {
        if self.l_panel.active {
            self.l_panel.index = index;
        } else {
            self.r_panel.index = index;
        }
    }

    pub fn reload(&mut self, position: PanelPosition) {
        match position {
            PanelPosition::L => {
                self.l_panel.items = read_items(&self.l_panel.path);
            },
            PanelPosition::R => {
                self.r_panel.items = read_items(&self.r_panel.path);
            },
        }
    }

    pub fn cd(&mut self, dir: String) {
        if self.l_panel.active {
            self.l_panel.path.push(dir);
            self.l_panel.path = self.l_panel.path.canonicalize().expect("Error while cannonize path");
            self.l_panel.index = 0;
            self.l_panel.items = read_items(&self.l_panel.path);
        } else {
            self.r_panel.path.push(dir);
            self.r_panel.path = self.r_panel.path.canonicalize().expect("Error while cannonize path");
            self.r_panel.index = 0;
            self.r_panel.items = read_items(&self.r_panel.path);
        }
    }

    pub fn add_index(&mut self, times: usize) {
        let panel = self.active_panel();
        let new_index = panel.index.saturating_add(times) % panel.items.len();
        if new_index > panel.index {
            self.set_active_panel_index(new_index);
        } else {
            self.set_active_panel_index(panel.items.len() - 1);
        }
    }

    pub fn sub_index(&mut self, times: usize) {
        self.set_active_panel_index(
            self.active_panel().index.saturating_sub(times) % self.active_panel().items.len(),
        );
    }
}

fn read_items(path: &Path) -> Vec<PanelItem> {
    let paths = read_dir(path).unwrap();
    let mut items: Vec<PanelItem> = paths
        .filter_map(|path| Some(PanelItem::new(path.ok()?.path())))
        .collect();
    let mut back_path = PathBuf::new();
    back_path.push("..");
    items.push(PanelItem::new(back_path));
    items.sort_by(|pb_a, pb_b| pb_a.display_string().cmp(&pb_b.display_string()));
    items.sort_by_key(|pb| !pb.is_dir());
    items
}
