use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Widget, Block};

pub struct HelpLine {}

impl HelpLine {
    pub fn new() -> Self {
        HelpLine {}
    }
}

impl Widget for HelpLine {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Welcome to FIR (press 'q' to quit, 'tab' to switch panels)")
            .block(Block::default())
            .render(area, buf);
    }
}
