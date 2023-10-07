use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Widget};

use crate::app_context::AppContext;

pub struct HelpLine<'a> {
    context: &'a AppContext<'a>,
}

impl<'a> HelpLine<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        HelpLine { context }
    }
}

impl Widget for HelpLine<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let message = if self.context.editor_context().is_open() {
            Line::from(vec![
                Span::raw("ESC "),
                Span::styled("Quit", Style::default().bg(Color::Blue)),
                Span::raw(" ^s"),
                Span::styled("Save", Style::default().bg(Color::Blue)),
            ])
        } else {
            Line::from(vec![
                Span::raw(" ←→↑↓"),
                Span::styled("Navigate", Style::default().bg(Color::Blue)),
                Span::raw(" ↹"),
                Span::styled("Switch panel", Style::default().bg(Color::Blue)),
                Span::raw(" 4"),
                Span::styled("Edit", Style::default().bg(Color::Blue)),
                Span::raw(" 10"),
                Span::styled("Quit", Style::default().bg(Color::Blue)),
            ])
        };
        Paragraph::new(message)
            .block(Block::default())
            .render(area, buf);
    }
}
