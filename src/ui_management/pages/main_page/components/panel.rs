use std::{path::PathBuf, time::Duration};

use crossterm::event::KeyCode;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem, ListState, StatefulWidget},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    state_management::{action::Action, PanelItem, PanelPosition},
    ui_management::components::{Component, ComponentRender},
};

struct Props {
    active: bool,
    items: Vec<PanelItem>,
    pub directory: Option<PathBuf>,
    index: usize,
}

pub struct Panel {
    pub action_tx: UnboundedSender<Action>,
    props: Props,
    placement: PanelPosition,
    #[allow(dead_code)]
    watcher: RecommendedWatcher,
    watch_rx: std::sync::mpsc::Receiver<Result<notify::Event, notify::Error>>,
}

impl Panel {
    pub fn set_placement(&mut self, placement: PanelPosition) {
        self.placement = placement;
    }

    pub fn current_item(&mut self) -> &PanelItem {
        self.props
            .items
            .get(self.props.index)
            .expect("Index points on nonexistent file")
    }

    pub fn current_full_path(&mut self) -> String {
        self.current_item().file_full_path()
    }

    pub fn current_dir(&self) -> PathBuf {
        self.props
            .directory
            .clone()
            .expect("No directory set for panel")
    }

    fn current_file_name(&mut self) -> String {
        self.current_item().current_file_name()
    }
}

impl Component for Panel {
    fn new(
        state: &crate::state_management::State,
        action_tx: tokio::sync::mpsc::UnboundedSender<crate::state_management::action::Action>,
    ) -> Self
    where
        Self: Sized,
    {
        let (watch_tx, watch_rx) =
            std::sync::mpsc::channel::<Result<notify::Event, notify::Error>>();
        let watcher = RecommendedWatcher::new(watch_tx, notify::Config::default())
            .expect("Faield to create watcher");

        Panel {
            action_tx: action_tx.clone(),
            props: Props {
                active: true,
                directory: None,
                items: vec![],
                index: 0,
            },
            watcher,
            watch_rx,
            placement: PanelPosition::L,
        }
        .move_with_state(state)
    }

    fn move_with_state(self, state: &crate::state_management::State) -> Self
    where
        Self: Sized,
    {
        let my_state = match self.placement {
            PanelPosition::R => &state.r_panel,
            PanelPosition::L => &state.l_panel,
        };

        let (watch_tx, watch_rx) =
            std::sync::mpsc::channel::<Result<notify::Event, notify::Error>>();
        let mut watcher = RecommendedWatcher::new(watch_tx, notify::Config::default())
            .expect("Faield to create watcher");
        watcher
            .watch(my_state.path.as_ref(), RecursiveMode::NonRecursive)
            .expect("Failed to watch file event");

        Self {
            props: Props {
                directory: Some(my_state.path.clone()),
                items: my_state.items.clone(),
                active: my_state.active,
                index: my_state.index,
            },
            watcher,
            watch_rx,
            ..self
        }
    }

    fn name(&self) -> &str {
        match self.placement {
            PanelPosition::L => "Left panel",
            PanelPosition::R => "Right panel",
        }
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Down => {
                let _ = self.action_tx.send(Action::FileItemDown(1));
            }
            KeyCode::Up => {
                let _ = self.action_tx.send(Action::FileItemUp(1));
            }
            KeyCode::PageDown => {
                let _ = self.action_tx.send(Action::FileItemDown(20));
            }
            KeyCode::PageUp => {
                let _ = self.action_tx.send(Action::FileItemUp(20));
            }
            KeyCode::Right => {
                let _ = self.action_tx.send(Action::FileItemDown(20));
            }
            KeyCode::Left => {
                let _ = self.action_tx.send(Action::FileItemUp(20));
            }
            KeyCode::F(4) => {
                if !self.current_item().is_dir() {
                    let file_to_edit = self.current_full_path().clone();
                    let _ = self.action_tx.send(Action::Edit(file_to_edit));
                }
            }
            KeyCode::Enter => {
                if self.current_item().is_dir() {
                    let newdir = self.current_file_name().clone();
                    let _ = self.action_tx.send(Action::Cd(newdir));
                } else {
                    let file_to_open = self.current_full_path().clone();
                    let _ = self.action_tx.send(Action::Open(file_to_open));
                }
            }
            _ => {}
        }
    }

    fn check(&mut self) {
        let maybe_change = self.watch_rx.recv_timeout(Duration::from_millis(1));
        if maybe_change.is_ok() {
            let _ = self.action_tx.send(Action::Reload(self.placement.clone()));
        }
    }
}

pub struct RenderProps {
    pub area: Rect,
}

impl ComponentRender<RenderProps> for Panel {
    fn render(&self, frame: &mut ratatui::prelude::Frame, props: RenderProps) {
        let title_text = match &self.props.directory {
            None => String::from(""),
            Some(dir) => dir.display().to_string(),
        };

        let panel_items: Vec<ListItem> = self.props.items.iter().map(|item| item.into()).collect();

        let mut title_style = Style::new();
        if self.props.active {
            title_style = title_style.bg(Color::LightCyan).fg(Color::Black);
        }
        let panel_block = Block::bordered().border_type(BorderType::Double).border_style(Style::default())
            .title_style(title_style)
            .title(title_text)
            .title_alignment(Alignment::Center)
            .style(Style::new().bg(Color::Black).fg(Color::White));
        let panel_list = List::new(panel_items)
            .block(panel_block)
            .highlight_style(Style::new().bg(Color::Cyan).fg(Color::Black));
        let mut state = ListState::default().with_selected(if self.props.active {
            Some(self.props.index)
        } else {
            None
        });
        StatefulWidget::render(panel_list, props.area, frame.buffer_mut(), &mut state);
    }
}
