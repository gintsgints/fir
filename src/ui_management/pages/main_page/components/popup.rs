use crate::{
    state_management::{action::Action, PopupType, State},
    ui_management::components::{Component, ComponentRender},
};
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};
use tokio::sync::mpsc::UnboundedSender;

use super::Button;

struct Props {
    pub popup_msg: String,
    pub popup_type: PopupType,
    pub active: bool,
    pub popup_next_action: Option<Action>,
}

impl From<&State> for Props {
    fn from(value: &State) -> Self {
        Props {
            popup_next_action: value.popup_next_action.clone(),
            active: !value.popup_msg.is_empty(),
            popup_msg: value.popup_msg.clone(),
            popup_type: value.popup_type.clone(),
        }
    }
}

pub struct Popup {
    pub action_tx: UnboundedSender<Action>,
    props: Props,
    ok_button: Button,
    cancel_button: Button,
}

impl Popup {
    pub fn active(&self) -> bool {
        self.props.active
    }

    /// # Usage
    ///
    /// ```rust
    /// let rect = centered_rect(f.size(), 50, 50);
    /// ```
    fn centered_rect(&self, r: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

impl Component for Popup {
    fn new(
        state: &crate::state_management::State,
        action_tx: tokio::sync::mpsc::UnboundedSender<crate::state_management::action::Action>,
    ) -> Self
    where
        Self: Sized,
    {
        let mut ok_button = Button::new(state, action_tx.clone()).title(String::from("Ok"));
        ok_button.active = true;
        let cancel_button = Button::new(state, action_tx.clone()).title(String::from("Cancel"));
        Popup {
            action_tx,
            props: Props::from(state),
            ok_button,
            cancel_button,
        }
    }

    fn move_with_state(self, state: &crate::state_management::State) -> Self
    where
        Self: Sized,
    {
        Self {
            props: Props::from(state),
            ..self
        }
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Right => {
                if self.ok_button.active {
                    self.ok_button.active = false;
                    self.cancel_button.active = true;
                }
            }
            KeyCode::Left => {
                if self.cancel_button.active {
                    self.ok_button.active = true;
                    self.cancel_button.active = false;
                }
            }
            KeyCode::Esc => {
                let _ = self.action_tx.send(Action::Cancel);
            }
            KeyCode::Enter => {
                if (self.props.popup_type == PopupType::Error) || (self.cancel_button.active) {
                    let _ = self.action_tx.send(Action::Cancel);
                } else if let Some(action) = self.props.popup_next_action.clone() {
                    let _ = self.action_tx.send(action);
                }
            }
            _ => {}
        };
    }

    fn check(&mut self) {
        todo!()
    }
}

pub struct PopupRenderProps {
    pub area: Rect,
}

impl ComponentRender<PopupRenderProps> for Popup {
    fn render(&self, frame: &mut ratatui::prelude::Frame, props: PopupRenderProps) {
        if self.props.active {
            let popup_area = self.centered_rect(props.area, 60, 20);
            let popup_text_area = self.centered_rect(props.area, 50, 15);
            frame.render_widget(Clear, popup_area);
            let [msg_rec, button_rec] = *Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(popup_text_area)
            else {
                panic!("Popup should have 2 chunks")
            };
            let block = Block::default()
                .borders(Borders::all())
                .style(Style::default().bg(Color::White).fg(Color::Black))
                .title(if self.props.popup_type == PopupType::Error {
                    "Error"
                } else {
                    "Please select"
                });
            frame.render_widget(block, popup_area);
            frame.render_widget(Paragraph::new(self.props.popup_msg.clone()).wrap(Wrap {trim: false}), msg_rec);
            match self.props.popup_type {
                PopupType::Error => {
                    self.ok_button
                        .render(frame, super::button::RenderProps { area: button_rec });
                }
                PopupType::YesNo => {
                    let button_recs = Layout::horizontal([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .split(button_rec);
                    self.ok_button.render(
                        frame,
                        super::button::RenderProps {
                            area: button_recs[0],
                        },
                    );
                    self.cancel_button.render(
                        frame,
                        super::button::RenderProps {
                            area: button_recs[1],
                        },
                    );
                }
            }
        }
    }
}
