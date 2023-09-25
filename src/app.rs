use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crate::{errors::ProgramError, root::Root, app_context::AppContext};

pub fn run() -> Result<(), ProgramError> {
    let mut terminal = setup_terminal()?;
    let context = AppContext::new(".", ".")?;
    loop {
        terminal.draw(|frame|frame.render_widget(Root::new(&context), frame.size()))?;
        if should_quit()? {
            break;
        }
    }
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, ProgramError> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), ProgramError> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(terminal.show_cursor()?)
}

fn should_quit() -> Result<bool, ProgramError> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
