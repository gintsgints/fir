use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Padding, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    state_management::action::Action,
    ui_management::components::{Component, ComponentRender},
};

pub struct Button {
    action_tx: UnboundedSender<Action>,
    pub active: bool,
    title: String,
}

impl Button {
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
}

impl Component for Button {
    fn new(
        _state: &crate::state_management::State,
        action_tx: tokio::sync::mpsc::UnboundedSender<crate::state_management::action::Action>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            active: false,
            title: String::from("Ok"),
            action_tx,
        }
    }

    fn move_with_state(self, _state: &crate::state_management::State) -> Self
    where
        Self: Sized,
    {
        Self { ..self }
    }

    fn name(&self) -> &str {
        "button"
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        if self.active && (key.code == KeyCode::Enter || key.code == KeyCode::Char(' ')) {
            let _ = self.action_tx.send(Action::Cancel);
        }
    }

    fn check(&mut self) {
        todo!()
    }
}

pub struct RenderProps {
    pub area: Rect,
}

impl ComponentRender<RenderProps> for Button {
    fn render(&self, frame: &mut ratatui::prelude::Frame, props: RenderProps) {
        let style = match self.active {
            true => Style::new().bg(Color::Cyan),
            false => Style::new().bg(Color::White),
        };
        let block = Block::new().style(style);
        let paragraph = Paragraph::new(self.title.clone())
            .block(block)
            .alignment(Alignment::Center);
        let outer_block = Block::new().padding(Padding {
            left: 1,
            right: 1,
            top: 0,
            bottom: 1,
        });
        let inner = outer_block.inner(props.area);

        frame.render_widget(outer_block, props.area);
        frame.render_widget(paragraph, inner);
    }
}
