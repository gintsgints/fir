use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;
use tui_textarea::TextArea;

use crate::{
    state_management::{action::Action, State},
    ui_management::components::{Component, ComponentRender},
};

struct Props {
    modified: bool,
    editor_file: String,
}

impl From<&State> for Props {
    fn from(value: &State) -> Self {
        let editor_file = if let Some(filename) = value.editor_file.clone() {
            filename.clone()
        } else {
            "".to_string()
        };
        Props {
            editor_file,
            modified: value.editor_modified,
        }
    }
}

pub struct EditorPage<'a> {
    props: Props,
    text_area: Option<TextArea<'a>>,
    pub action_tx: UnboundedSender<Action>,
}

impl<'a> Component for EditorPage<'a> {
    fn new(
        state: &crate::state_management::State,
        action_tx: tokio::sync::mpsc::UnboundedSender<crate::state_management::action::Action>,
    ) -> Self
    where
        Self: Sized,
    {
        let props = Props::from(state);
        EditorPage {
            text_area: None,
            action_tx: action_tx.clone(),
            props,
        }
        .move_with_state(state)
    }

    fn move_with_state(self, state: &crate::state_management::State) -> EditorPage<'a>
    where
        Self: Sized,
    {
        let props = Props::from(state);
        if !props.editor_file.is_empty() && self.text_area.is_none() {
            let mut text_area: TextArea =
                BufReader::new(File::open(props.editor_file.clone()).expect("Can't read file"))
                    .lines()
                    .collect::<Result<_, _>>()
                    .expect("Error?");
            if text_area.lines().iter().any(|l| l.starts_with('\t')) {
                text_area.set_hard_tab_indent(true);
            };
            return EditorPage {
                props,
                text_area: Some(text_area),
                ..self
            };
        };
        EditorPage { props, ..self }
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.text_area = None;
                let _ = self.action_tx.send(Action::EditorExit);
            }
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.props.modified {
                    if let Some(text_area) = &self.text_area {
                        let mut f = BufWriter::new(
                            File::create(&self.props.editor_file).expect("Cannot open file"),
                        );
                        for line in text_area.lines() {
                            f.write_all(line.as_bytes())
                                .expect("Not able to write linee");
                            f.write_all(b"\n").expect("Not able to write endline");
                        }
                        let _ = self.action_tx.send(Action::EditorResedModified);
                    }
                }
            }
            _ => {
                if let Some(text_area) = self.text_area.as_mut() {
                    let modified = text_area.input(key);
                    if modified {
                        let _ = self.action_tx.send(Action::EditorModified);
                    }
                }
            }
        }
    }
    
    fn check(&mut self) {
        
    }
}

pub struct RenderProps {
    pub area: Rect,
}

impl<'a> ComponentRender<RenderProps> for EditorPage<'a> {
    fn render(&self, frame: &mut ratatui::prelude::Frame, props: RenderProps) {
        if let Some(text_area) = &self.text_area {
            frame.render_widget(text_area.widget(), props.area)
        }
    }
}
