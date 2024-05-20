use std::{path::MAIN_SEPARATOR, process::{Command, Output}};

use anyhow::Ok;
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

use crate::termination::{Interrupted, Terminator};

use super::{action::Action, State};

#[derive(PartialEq, Debug, Clone)]
pub enum PanelPosition {
    L,
    R,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum PopupType {
    Input,
    YesNo,
    #[default]
    Error,
}

pub struct StateStore {
    state_tx: UnboundedSender<State>,
}

impl<'a> StateStore {
    pub fn new() -> (Self, UnboundedReceiver<State>) {
        let (state_tx, state_rx) = mpsc::unbounded_channel::<State>();

        (StateStore { state_tx }, state_rx)
    }

    pub async fn main_loop(
        self,
        mut terminator: Terminator,
        mut action_rx: UnboundedReceiver<Action>,
        mut interrupt_rx: broadcast::Receiver<Interrupted>,
    ) -> anyhow::Result<Interrupted> {
        let mut state: State = State::new()?;

        // the initial state once
        self.state_tx.send(state.clone())?;

        let result = loop {
            tokio::select! {
                Some(action) = action_rx.recv() => match action {
                    Action::SwitchTabs => {
                        state.l_panel.active = !state.l_panel.active;
                        state.r_panel.active = !state.r_panel.active;
                    },
                    Action::Reload(panel_position) => {
                        state.reload(panel_position);
                    },
                    Action::FileItemDown(times) => {
                        state.add_index(times);
                    },
                    Action::FileItemUp(times) => {
                        state.sub_index(times);
                    },
                    Action::Cd(dir) => {
                        state.cd(dir);
                    },
                    Action::Copy(from, to) => {
                        #[cfg(target_os = "windows")]
                        let output = Command::new("copy")
                            .arg(from)
                            .arg(to)
                            .output()
                            .expect("Filed to copy file");
                        #[cfg(not(target_os = "windows"))]
                        let output = Command::new("cp")
                            .arg("-r")
                            .arg(from)
                            .arg(to)
                            .output()
                            .expect("Filed to copy file");
                        if !output.status.success() {
                            state.popup_msg = self.get_error_msg(&output)?;
                            state.popup_type = PopupType::Error;
                        }
                        state.reload(PanelPosition::L);
                        state.reload(PanelPosition::R);
                    },
                    Action::MkDirInput => {
                        state.popup_msg = String::from("Create directory:");
                        state.popup_type = PopupType::Input;
                        state.popup_next_action = Some(Action::MkDir)
                    },
                    Action::SetInput(value) => {
                        state.popup_input = value;
                    },
                    Action::MkDir => {
                        state.popup_next_action = None;
                        let mut full_path = if state.l_panel.active {
                            state.l_panel.path.display().to_string().clone()
                        } else {
                            state.r_panel.path.display().to_string().clone()
                        };
                        full_path.push(MAIN_SEPARATOR);
                        full_path.push_str(&state.popup_input);
                        #[cfg(target_os = "windows")]
                        let output = Command::new("md")
                            .arg(full_path)
                            .output()
                            .expect("Filed to create directory");
                        #[cfg(not(target_os = "windows"))]
                        let output = 
                        Command::new("mkdir")
                            .arg(full_path)
                            .output()
                            .expect("Filed to create directory");
                        if !output.status.success() {
                            state.popup_msg = self.get_error_msg(&output)?;
                            state.popup_type = PopupType::Error;
                        } else {
                            state.popup_msg = String::from("");
                        }
                    },
                    Action::RmYesNo(file) => {
                        let mut msg = String::from("Do you want to remove file? ");
                        msg.push_str(&file);
                        state.popup_msg = msg;
                        state.popup_type = PopupType::YesNo;
                        state.popup_next_action = Some(Action::Rm(file))
                    }
                    Action::Rm(file) => {
                        state.popup_next_action = None;
                        #[cfg(target_os = "windows")]
                        let output = Command::new("del")
                            .arg(file)
                            .output()
                            .expect("Filed to remove file");
                        #[cfg(not(target_os = "windows"))]
                        let output = Command::new("rm")
                            .arg("-rf")
                            .arg(file)
                            .output()
                            .expect("Filed to remove file");
                        if !output.status.success() {
                            state.popup_msg = self.get_error_msg(&output)?;
                            state.popup_type = PopupType::Error;
                        } else {
                            state.popup_msg = String::from("");
                        }
                    },
                    Action::Open(file) => {
                        #[cfg(target_os = "windows")]
                        let output = Command::new("cmd")
                            .args(["/C", "start ", file])
                            .output()
                            .expect("Failed to open file");
                        #[cfg(not(target_os = "windows"))]
                        let output = Command::new("open")
                            .arg(file)
                            .output()
                            .expect("Failed to open file");
                        if !output.status.success() {
                            state.popup_msg = self.get_error_msg(&output)?;
                            state.popup_type = PopupType::Error;
                        }
                    },
                    Action::Edit(file) => {
                        state.editor_file = Some(file);
                        state.editor_modified = false;
                    },
                    Action::EditorResedModified => {
                        state.editor_modified = false;
                    },
                    Action::EditorModified => {
                        state.editor_modified = true;
                    },
                    Action::EditorExit => {
                        state.editor_file = None;
                    },
                    Action::Cancel => {
                        state.popup_msg = String::from("");
                        state.popup_next_action = None;
                    },
                    Action::Exit => {
                        let _ = terminator.terminate(Interrupted::UserInt);

                        break Interrupted::UserInt;
                    }
                },
                // Catch and handle interrupt signal to gracefully shutdown
                core::result::Result::Ok(interrupted) = interrupt_rx.recv() => {
                    break interrupted;
                }
            }

            self.state_tx.send(state.clone())?;
        };

        Ok(result)
    }

    fn get_error_msg(&self, output: &Output) -> anyhow::Result<String> {
        let msg = format!("{}", String::from_utf8_lossy(&output.stderr));
        Ok(msg)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_error_msg_test() {
        let (state_store, _state_rx) = StateStore::new();
        let output = Command::new("/bin/cat")
            .arg("file.txt")
            .output()
            .expect("failed to execute process");
        let msg = state_store.get_error_msg(&output).unwrap();
        assert_eq!(msg, "/bin/cat: file.txt: No such file or directory\n");
    }
}
