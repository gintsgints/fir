use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Result, Write};
use std::path::PathBuf;

use tui_textarea::{Input, TextArea};

#[derive(Clone)]
pub struct EditorContext<'a> {
    pub textarea: TextArea<'a>,
    path: String,
    modified: bool,
    active: bool,
}

impl<'a> EditorContext<'a> {
    pub fn new(path: String) -> Result<Self> {
        let mut textarea: TextArea = BufReader::new(File::open(&path)?)
            .lines()
            .collect::<Result<_>>()?;
        if textarea.lines().iter().any(|l| l.starts_with('\t')) {
            textarea.set_hard_tab_indent(true);
        }
        Ok(Self {
            textarea,
            path,
            modified: false,
            active: false,
        })
    }

    fn save(&mut self) -> Result<()> {
        if !self.modified {
            return Ok(());
        }
        let mut f = BufWriter::new(File::create(&self.path)?);
        for line in self.textarea.lines() {
            f.write_all(line.as_bytes())?;
            f.write_all(b"\n")?;
        }
        self.modified = false;
        Ok(())
    }

    pub fn input(&mut self, input: impl Into<Input>) -> bool {
        self.textarea.input(input)
    }
}
