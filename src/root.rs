use ratatui::{
    prelude::{Buffer, Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use crate::{app_context::AppContext, help_line::HelpLine, panel::Panel, editor::Editor};

pub struct Root<'a> {
    context: &'a AppContext<'a>,
}

impl<'a> Root<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Root { context }
    }
}

impl Widget for Root<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.context.editor_context().is_open() {
            let editor = Editor::new(self.context.editor_context());
            editor.render(area, buf);
            return;
        }
        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);
        let panels = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main[0]);

        let l_panel = Panel::new(self.context.left_context());
        l_panel.render(panels[0], buf);
        let r_panel = Panel::new(self.context.right_context());
        r_panel.render(panels[1], buf);
        HelpLine::new(&self.context).render(main[1], buf);
    }
}
