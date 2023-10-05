use ratatui::widgets::Widget;

use crate::app_context::editor_context::EditorContext;

pub struct Editor<'a> {
    context: &'a EditorContext<'a>,
}

impl<'a> Editor<'a> {
    pub fn new(context: &'a EditorContext) -> Self {
        Editor { context }
    }
}

impl Widget for Editor<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if let Some(textarea) = &self.context.textarea {
            textarea.widget().render(area, buf);
            return;
        }
    }
}
