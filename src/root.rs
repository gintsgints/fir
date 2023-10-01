use std::{ops::Deref, path::PathBuf};

use ratatui::{
    prelude::{Alignment, Buffer, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        block::Title, Block, BorderType, Borders, List, ListItem, ListState, Paragraph,
        StatefulWidget, Widget,
    },
};

use crate::app_context::AppContext;

pub struct Root<'a> {
    context: &'a AppContext,
}

impl<'a> Root<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Root { context }
    }

    fn create_file_item(&self, fb: &PathBuf) -> ListItem {
        let style = if fb.is_dir() {
            Style::new().fg(Color::White)
        } else {
            Style::new().fg(Color::Cyan)
        };
        let file_name = fb.file_name().expect("msg").to_str().expect("msg");
        let file_line = Line::from(vec![Span::styled(file_name.to_string(), style)]);
        ListItem::new(vec![file_line])
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
        let dir_files_r: Vec<ListItem> = self
            .context
            .right_files()
            .iter()
            .map(|fb| self.create_file_item(fb))
            .collect();
        let panel_r = List::new(dir_files_r)
            .block(
                Block::default()
                    .title(Title::from(format!(" {} ", dir_r.deref())).alignment(Alignment::Center))
                    .title_style(Style::new().bg(Color::Cyan).fg(Color::Black))
                    .border_type(BorderType::Double)
                    .borders(Borders::all())
                    .style(Style::new().bg(Color::Blue)),
            )
            .highlight_style(Style::new().bg(Color::Cyan));

        let dir_l = self.context.left_path();
        let dir_files_l: Vec<ListItem> = self
            .context
            .left_files()
            .iter()
            .map(|fb| self.create_file_item(fb))
            .collect();
        let panel_l = List::new(dir_files_l)
            .block(
                Block::default()
                    .title(Title::from(format!(" {} ", dir_l.deref())).alignment(Alignment::Center))
                    .title_style(Style::new().bg(Color::Cyan).fg(Color::Black))
                    .border_type(BorderType::Double)
                    .borders(Borders::all())
                    .style(Style::new().bg(Color::Blue)),
            )
            .highlight_style(Style::new().bg(Color::Cyan));

        let help = Block::default();
        let greeting = Paragraph::new("Welcome to FIR (press 'q' to quit, 'tab' to switch panels)");

        let mut l_state = ListState::default().with_selected(self.context.left_selection_index());
        let mut r_state = ListState::default().with_selected(self.context.right_selection_index());
        StatefulWidget::render(panel_l, panels[0], buf, &mut l_state);
        StatefulWidget::render(panel_r, panels[1], buf, &mut r_state);
        greeting.clone().block(help).render(main[1], buf);
    }
}
