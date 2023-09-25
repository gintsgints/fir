use std::ops::Deref;

use ratatui::{
    prelude::{Buffer, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, Paragraph, Widget, ListItem}, text::Line,
};

use crate::app_context::AppContext;

pub struct Root<'a> {
    context: &'a AppContext,
}

impl<'a> Root<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Root { context }
    }
}

impl Widget for Root<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);
        let panels = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main[0]);

        let dir_r = self.context.right_path();
        let dir_files_r: Vec<ListItem> = self.context.right_files().iter().map(|fb| {
            ListItem::new(vec![Line::from(fb.display().to_string())])
        }).collect();
        let panel_r = List::new(dir_files_r).block(
            Block::default()
                .title(dir_r.deref())
                .borders(Borders::all()),
        );

        let dir_l = self.context.left_path();
        let dir_files_l: Vec<ListItem> = self.context.right_files().iter().map(|fb| {
            ListItem::new(vec![Line::from(fb.display().to_string())])
        }).collect();
        let panel_l = List::new(dir_files_l).block(
            Block::default()
                .title(dir_l.deref())
                .borders(Borders::all()),
        );

        let help = Block::default();
        let greeting = Paragraph::new("Welcome to FIR (press 'q' to quit)");

        panel_l.render(panels[0], buf);
        panel_r.render(panels[1], buf);
        greeting.clone().block(help).render(main[1], buf);
    }
}
