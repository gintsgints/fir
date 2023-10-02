use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crate::{app_context::AppContext, errors::ProgramError, root::Root};

pub struct App {
    context: AppContext,
}

impl App {
    pub fn new() -> Result<Self, ProgramError> {
        let context = AppContext::new()?;
        Ok(App { context })
    }

    pub fn run(&mut self) -> Result<(), ProgramError> {
        let mut terminal = self.setup_terminal()?;
        while !self.context.should_quit {
            terminal.draw(|frame| frame.render_widget(Root::new(&self.context), frame.size()))?;
            self.handle_events()?;
        }
        self.restore_terminal(&mut terminal)?;
        Ok(())
    }

    fn setup_terminal(&self) -> Result<Terminal<CrosstermBackend<Stdout>>, ProgramError> {
        let mut stdout = stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        Ok(Terminal::new(CrosstermBackend::new(stdout))?)
    }

    fn restore_terminal(
        &self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), ProgramError> {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(terminal.show_cursor()?)
    }

    fn handle_events(&mut self) -> Result<(), ProgramError> {
        if event::poll(Duration::from_millis(250))? {
            let key_event = event::read()?;
            match key_event {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char('q') => self.context.should_quit = true,
                        KeyCode::Enter => self.context.apply_cmd(crate::commands::Command::cd)?,
                        KeyCode::Up => self.context.key_up(),
                        KeyCode::Down => self.context.key_down(),
                        KeyCode::Tab => self.context.tab(),
                        _ => {}
                    }
                    return Ok(())
                },
                _ => return Ok(()),
            }
        };
        Ok(())
    }
}
