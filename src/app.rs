use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::{stdout, Stdout},
    time::Duration,
};
use anyhow::Result;

use crate::{app_context::AppContext, commands::AppCommand::*, root::Root};

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<Stdout>>;

pub struct App<'a> {
    context: AppContext<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self> {
        let context = AppContext::new()?;
        Ok(App { context })
    }

    pub fn run(&mut self) -> Result<()> {
        self.startup()?;

        let status = self.main_loop();
        self.shutdown()?;
        status?;
        Ok(())
    }

    fn main_loop(&mut self) -> Result<()> {
        let mut t = Terminal::new(CrosstermBackend::new(stdout()))?;
        loop {
            // application render
            t.draw(|f| {
                self.ui(f);
            })?;

            // application update
            self.update()?;

            // application exit
            if self.context.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn startup(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn ui(&mut self, f: &mut Frame<'_>) {
        f.render_widget(Root::new(&self.context), f.size())
    }

    fn update(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(250))? {
            if self.context.editor_context().is_open() {
                self.context.editor_update()?;
                return Ok(());
            }
            let key_event = event::read()?;
            match key_event {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
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
                            KeyCode::F(4) => {
                                if !self.context.current_item_is_dir() {
                                    self.context.apply_cmd(Edit)?
                                }
                            }
                            KeyCode::F(10) => self.context.should_quit = true,
                            _ => {}
                        }
                    }
                    return Ok(());
                }
                _ => return Ok(()),
            }
        };
        Ok(())
    }
}
