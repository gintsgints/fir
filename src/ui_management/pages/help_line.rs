use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::{
    state_management::State,
    ui_management::components::{Component, ComponentRender},
};

struct Props {
    editor_active: bool,
    editor_modified: bool,
}

impl From<&State> for Props {
    fn from(value: &State) -> Self {
        Props {
            editor_active: value.editor_file.is_some(),
            editor_modified: value.editor_modified,
        }
    }
}

pub struct HelpLine {
    props: Props,
}

impl Component for HelpLine {
    fn new(
        state: &crate::state_management::State,
        _action_tx: tokio::sync::mpsc::UnboundedSender<crate::state_management::action::Action>,
    ) -> Self
    where
        Self: Sized,
    {
        HelpLine {
            props: Props::from(state),
        }
        .move_with_state(state)
    }

    fn move_with_state(self, state: &crate::state_management::State) -> Self
    where
        Self: Sized,
    {
        Self {
            props: Props::from(state),
        }
    }

    fn name(&self) -> &str {
        "Help line"
    }

    fn handle_key_event(&mut self, _key: crossterm::event::KeyEvent) {}

    fn check(&mut self) {}
}

pub struct RenderProps {
    pub area: Rect,
}

impl ComponentRender<RenderProps> for HelpLine {
    fn render(&self, frame: &mut ratatui::prelude::Frame, props: RenderProps) {
        let message = if !self.props.editor_active {
            Line::from(vec![
                Span::raw(" ←→↑↓"),
                Span::styled("Navigate", Style::default().bg(Color::Blue)),
                Span::raw(" ↹"),
                Span::styled("Switch panel", Style::default().bg(Color::Blue)),
                Span::raw(" 4"),
                Span::styled("Edit", Style::default().bg(Color::Blue)),
                Span::raw(" 5"),
                Span::styled("Copy", Style::default().bg(Color::Blue)),
                Span::raw(" 6"),
                Span::styled("Move", Style::default().bg(Color::Blue)),
                Span::raw(" 7"),
                Span::styled("MkDir", Style::default().bg(Color::Blue)),
                Span::raw(" 8"),
                Span::styled("Delete", Style::default().bg(Color::Blue)),
                Span::raw(" 10"),
                Span::styled("Quit", Style::default().bg(Color::Blue)),
            ])
        } else {
            let mut line = Line::from(vec![
                Span::raw("ESC "),
                Span::styled("Quit", Style::default().bg(Color::Blue)),
            ]);
            if self.props.editor_modified {
                line.push_span(Span::raw(" ^s"));
                line.push_span(Span::styled("Save", Style::default().bg(Color::Blue)));
            }
            line
        };
        let line = Paragraph::new(message).block(Block::default());

        frame.render_widget(line, props.area);
    }
}
