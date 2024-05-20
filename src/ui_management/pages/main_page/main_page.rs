use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    state_management::{action::Action, PanelItem, PanelPosition, State},
    ui_management::components::{Component, ComponentRender},
};

use super::{
    components::{Popup, PopupRenderProps},
    Panel,
};

struct Props {
    active_panel: PanelPosition,
}

impl From<&State> for Props {
    fn from(value: &State) -> Self {
        Props {
            active_panel: if value.r_panel.active {
                PanelPosition::R
            } else {
                PanelPosition::L
            },
        }
    }
}

pub struct MainPage<'a> {
    /// Action sender
    pub action_tx: UnboundedSender<Action>,
    // Mapped Props from State
    props: Props,
    // Panels
    l_panel: Panel,
    r_panel: Panel,
    popup: Popup<'a>,
}

impl<'a> MainPage<'a> {
    fn current_item(&mut self) -> &PanelItem {
        match self.props.active_panel {
            PanelPosition::L => self.l_panel.current_item(),
            PanelPosition::R => self.r_panel.current_item(),
        }
    }

    fn opposite_panel(&mut self) -> &Panel {
        match self.props.active_panel {
            PanelPosition::L => &self.r_panel,
            PanelPosition::R => &self.l_panel,
        }
    }

    fn opposite_path(&mut self) -> String {
        self.opposite_panel().current_dir().display().to_string()
    }
}

impl<'a> Component for MainPage<'a> {
    fn new(state: &crate::state_management::State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        let mut l_panel = Panel::new(state, action_tx.clone());
        l_panel.set_placement(PanelPosition::L);
        let mut r_panel = Panel::new(state, action_tx.clone());
        r_panel.set_placement(PanelPosition::R);
        let popup = Popup::new(state, action_tx.clone());
        MainPage {
            action_tx: action_tx.clone(),
            props: Props::from(state),
            l_panel,
            r_panel,
            popup,
        }
        .move_with_state(state)
    }

    fn move_with_state(self, state: &crate::state_management::State) -> Self
    where
        Self: Sized,
    {
        MainPage {
            props: Props::from(state),
            l_panel: self.l_panel.move_with_state(state),
            r_panel: self.r_panel.move_with_state(state),
            popup: self.popup.move_with_state(state),
            ..self
        }
    }

    fn name(&self) -> &str {
        "Main panels"
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        if self.popup.active() {
            self.popup.handle_key_event(key);
        } else {
            if key.kind != KeyEventKind::Press {
                return;
            }

            if self.props.active_panel == PanelPosition::L {
                self.l_panel.handle_key_event(key)
            } else {
                self.r_panel.handle_key_event(key)
            }

            match key.code {
                KeyCode::Tab => {
                    let _ = self.action_tx.send(Action::SwitchTabs);
                }
                KeyCode::F(4) => {
                    if !self.current_item().is_dir() {
                        let file_to_edit = self.current_item().current_file_name();
                        let _ = self.action_tx.send(Action::Edit(file_to_edit));
                    }
                }
                KeyCode::F(5) => {
                    let copy_from = self.current_item().file_full_path();
                    let copy_to = self.opposite_path();
                    let _ = self.action_tx.send(Action::Copy(copy_from, copy_to));
                }
                KeyCode::F(7) => {
                    let _ = self.action_tx.send(Action::MkDirInput);
                }
                KeyCode::F(8) => {
                    let remove = self.current_item().file_full_path();
                    let _ = self.action_tx.send(Action::RmYesNo(remove));
                }
                KeyCode::F(10) => {
                    let _ = self.action_tx.send(Action::Exit);
                }
                KeyCode::Char('d') => {
                    let remove = self.current_item().file_full_path();
                    let _ = self.action_tx.send(Action::RmYesNo(remove));
                }
                KeyCode::Char('q') => {
                    let _ = self.action_tx.send(Action::Exit);
                }
                KeyCode::Char('c') => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        let _ = self.action_tx.send(Action::Exit);
                    } else {
                        let copy_from = self.current_item().file_full_path();
                        let copy_to = self.opposite_path();
                        let _ = self.action_tx.send(Action::Copy(copy_from, copy_to));
                    }
                }
                _ => {}
            }
        }
    }

    fn check(&mut self) {
        self.l_panel.check();
        self.r_panel.check();
    }
}

pub struct RenderProps {
    pub area: Rect,
}

impl<'a> ComponentRender<RenderProps> for MainPage<'a> {
    fn render(&self, frame: &mut ratatui::prelude::Frame, props: RenderProps) {
        let [l_panel_rec, r_panel_rec] = *Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(props.area)
        else {
            panic!("The main layout should have 2 chunks")
        };

        self.l_panel
            .render(frame, super::components::RenderProps { area: l_panel_rec });
        self.r_panel
            .render(frame, super::components::RenderProps { area: r_panel_rec });
        self.popup
            .render(frame, PopupRenderProps { area: props.area })
    }
}
