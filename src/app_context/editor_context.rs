use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Result, Write};

use crossterm::event;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::block::Title;
use ratatui::widgets::{Block, BorderType, Borders};
use tui_textarea::{Input, Key, TextArea};

#[derive(Clone, Default)]
pub struct EditorContext<'a> {
    pub textarea: Option<TextArea<'a>>,
    path: String,
    modified: bool,
}

impl<'a> EditorContext<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_open(&self) -> bool {
        self.textarea.is_some()
    }

    pub fn open(&mut self, path: String) -> Result<()> {
        self.path = path;
        let mut textarea: TextArea = BufReader::new(File::open(&self.path)?)
            .lines()
            .collect::<Result<_>>()?;
        if textarea.lines().iter().any(|l| l.starts_with('\t')) {
            textarea.set_hard_tab_indent(true);
        };
        textarea.set_block(
            Block::default()
                .title(Title::from(format!(" {} ", self.path)).alignment(Alignment::Center))
                .title_style(Style::new().bg(Color::Cyan).fg(Color::Black))
                .border_type(BorderType::Double)
                .borders(Borders::all())
                .style(Style::new().bg(Color::Blue)),
        );
        self.textarea = Some(textarea);
        Ok(())
    }

    pub fn close(&mut self) {
        self.textarea = None
    }

    fn save(&mut self) -> Result<()> {
        if !self.modified {
            return Ok(());
        }
        let mut f = BufWriter::new(File::create(&self.path)?);
        if let Some(textarea) = &mut self.textarea {
            for line in textarea.lines() {
                f.write_all(line.as_bytes())?;
                f.write_all(b"\n")?;
            }
        }
        self.modified = false;
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        if let Some(textarea) = &mut self.textarea {
            match event::read()?.into() {
                Input { key: Key::Esc, .. } => {
                    self.close();
                }
                Input {
                    key: Key::Char('s'),
                    ctrl: true,
                    ..
                } => {
                    self.save()?;
                }
                input => {
                    self.modified = textarea.input(input);
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    pub fn input(&mut self, input: impl Into<Input>) -> bool {
        if let Some(textarea) = &mut self.textarea {
            textarea.input(input)
        } else {
            false
        }
    }
}
