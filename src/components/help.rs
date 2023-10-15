use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
  action::Action,
  config::{Config, KeyBindings},
};

#[derive(Default)]
pub struct Help {
  command_tx: Option<UnboundedSender<Action>>,
  editor_visible: bool,
  config: Config,
}

impl Help {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Component for Help {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    Ok(())
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => {},
      Action::Edit => self.editor_visible = true,
      Action::Exit => self.editor_visible = false,
      _ => {},
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let message = if self.editor_visible {
      Line::from(vec![
        Span::raw("ESC "),
        Span::styled("Quit", Style::default().bg(Color::Blue)),
        Span::raw(" ^s"),
        Span::styled("Save", Style::default().bg(Color::Blue)),
      ])
    } else {
      Line::from(vec![
        Span::raw(" ←→↑↓"),
        Span::styled("Navigate", Style::default().bg(Color::Blue)),
        Span::raw(" ↹"),
        Span::styled("Switch panel", Style::default().bg(Color::Blue)),
        Span::raw(" E"),
        Span::styled("Edit", Style::default().bg(Color::Blue)),
        Span::raw(" Q"),
        Span::styled("Quit", Style::default().bg(Color::Blue)),
      ])
    };

    f.render_widget(Paragraph::new(message).block(Block::default()), area);
    Ok(())
  }
}
