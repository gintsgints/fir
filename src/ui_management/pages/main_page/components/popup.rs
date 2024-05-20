use crate::{
    state_management::{action::Action, PopupType, State},
    ui_management::components::{Component, ComponentRender},
};
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap},
};
use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::TextArea;

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

pub struct Popup<'a> {
    pub action_tx: UnboundedSender<Action>,
    props: Props,
    ok_button: Button,
    cancel_button: Button,
    input: TextArea<'a>,
}

impl<'a> Popup<'a> {
    pub fn active(&self) -> bool {
        self.props.active
    }

    fn render_ok_button(&self, frame: &mut ratatui::prelude::Frame, area: Rect) {
        self.ok_button
        .render(frame, super::button::RenderProps { area });
    }

    fn render_ok_cancel_buttons(&self, frame: &mut ratatui::prelude::Frame, area: Rect) {
        let button_recs = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);
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

impl<'a> Component for Popup<'a> {
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
        let input_block = Block::default().padding(Padding { left: 0, right: 0, top: 0, bottom: 1 });
        let mut textarea = TextArea::default();
        textarea.set_block(input_block);
        textarea.set_style(Style::default().bg(Color::Blue).fg(Color::White));
        textarea.set_cursor_line_style(Style::default());
        textarea.set_cursor_style(Style::default().bg(Color::Cyan));
        Popup {
            action_tx,
            props: Props::from(state),
            input: textarea,
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
            _ => {
                if self.props.popup_type == PopupType::Input {
                    self.input.input(key);
                }
                let _ = self.action_tx.send(Action::SetInput(self.input.lines()[0].clone()));
            }
        };
    }

    fn check(&mut self) {
        todo!()
    }
}

pub struct PopupRenderProps {
    pub area: Rect,
}

impl<'a> ComponentRender<PopupRenderProps> for Popup<'a> {
    fn render(&self, frame: &mut ratatui::prelude::Frame, props: PopupRenderProps) {
        if self.props.active {
            let popup_area = self.centered_rect(props.area, 60, 20);
            let popup_text_area = self.centered_rect(props.area, 50, 15);
            frame.render_widget(Clear, popup_area);
            let [data_rec, button_rec] = *Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(2), Constraint::Length(2)])
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
            match self.props.popup_type {
                PopupType::Error => {
                    frame.render_widget(Paragraph::new(self.props.popup_msg.clone()).wrap(Wrap {trim: false}), data_rec);
                    self.render_ok_button(frame, button_rec);
                },
                PopupType::Input => {
                    let [msg_rec, input_rec] = *Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(data_rec)
                    else {
                        panic!("Data area should have 2 chungs")
                    };
                    frame.render_widget(Paragraph::new(self.props.popup_msg.clone()).wrap(Wrap {trim: false}), msg_rec);
                    frame.render_widget(self.input.widget(), input_rec);
                    self.render_ok_cancel_buttons(frame, button_rec)                    
                },
                PopupType::YesNo => {
                    frame.render_widget(Paragraph::new(self.props.popup_msg.clone()).wrap(Wrap {trim: false}), data_rec);
                    self.render_ok_cancel_buttons(frame, button_rec)
                }
            }
        }
    }
}
