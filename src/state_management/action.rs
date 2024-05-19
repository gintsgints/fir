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
    RmYesNo(String),
    Rm(String),
    SwitchTabs,
    Reload(PanelPosition),
    Cancel,
    Exit,
}
