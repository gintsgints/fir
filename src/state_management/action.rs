use super::state_store::PanelPosition;

#[derive(Debug, Clone)]
pub enum Action {
    /// Editor actions
    Edit(String),
    EditorResedModified,
    EditorModified,
    EditorExit,
    /// File panel actions
    FileItemUp(usize),
    FileItemDown(usize),
    Cd(String),
    Open(String),
    Copy(String, String),
    SetInput(String),
    RmYesNo(String),
    MkDirInput,
    Rm(String),
    MkDir,
    SwitchTabs,
    Reload(PanelPosition),
    Cancel,
    Exit,
}
