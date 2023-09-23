use ratatui::{backend::CrosstermBackend, widgets::Paragraph};
use std::io::Stdout;

pub fn render_app(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.size());
}
