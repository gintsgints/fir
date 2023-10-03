use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crate::{app_context::AppContext, commands::AppCommand::*, errors::ProgramError, root::Root};

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
                        KeyCode::Enter => {
                            if self.context.current_item_is_dir() {
                                self.context.apply_cmd(Cd)?
                            } else {
                                self.context.apply_cmd(Open)?
                            }
                        }
                        KeyCode::Char(' ') => self.context.key_down(1, true),
                        KeyCode::Up => self
                            .context
                            .key_up(1, key.modifiers.contains(KeyModifiers::SHIFT)),
                        KeyCode::Down => self
                            .context
                            .key_down(1, key.modifiers.contains(KeyModifiers::SHIFT)),
                        KeyCode::Left => self.context.key_up(20, false),
                        KeyCode::Right => self.context.key_down(20, false),
                        KeyCode::Tab => self.context.tab(),
                        _ => {}
                    }
                    return Ok(());
                }
                _ => return Ok(()),
            }
        };
        Ok(())
    }
}
