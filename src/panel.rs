use ratatui::{
    prelude::Alignment,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        block::Title, Block, BorderType, Borders, List, ListItem, ListState, StatefulWidget, Widget,
    },
};

use crate::{
    app_context::{panel_context::PanelContext, panel_item_context::PanelItemContext},
    filelist,
};

pub struct Panel<'a> {
    context: &'a PanelContext,
}

impl<'a> Panel<'a> {
    pub fn new(context: &'a PanelContext) -> Self {
        Panel { context }
    }

    fn create_file_item(&self, fb: &PanelItemContext) -> ListItem {
        let style = if *fb.marked() {
            Style::new().fg(Color::Yellow)
        } else {
            if fb.path().is_dir() {
                Style::new().fg(Color::White)
            } else {
                Style::new().fg(Color::Cyan)
            }
        };
        let file_name = filelist::file_name(&fb.path());
        let file_line = Line::from(vec![Span::styled(file_name.to_string(), style)]);
        ListItem::new(vec![file_line])
    }
}

impl Widget for Panel<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let dir_r = self.context.path_string();
        let dir_files_r: Vec<ListItem> = self
            .context
            .items()
            .iter()
            .map(|fb| self.create_file_item(fb))
            .collect();

        let mut title_style = Style::new();
        if self.context.active {
            title_style = title_style.bg(Color::Cyan).fg(Color::Black);
        }
        let panel = List::new(dir_files_r)
            .block(
                Block::default()
                    .title(Title::from(format!(" {} ", &dir_r)).alignment(Alignment::Center))
                    .title_style(title_style)
                    .border_type(BorderType::Double)
                    .borders(Borders::all())
                    .style(Style::new().bg(Color::Blue)),
            )
            .highlight_style(Style::new().bg(Color::Cyan).fg(Color::Black));
        let mut state = ListState::default().with_selected(if self.context.active {
            Some(self.context.index())
        } else {
            None
        });
        StatefulWidget::render(panel, area, buf, &mut state);
    }
}
