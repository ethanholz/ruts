use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RutConfig {
    pub workspaces: Option<Vec<WorkspaceConfig>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WorkspaceConfig {
    pub name: String,
    pub windows: Option<Vec<Window>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Window {
    pub name: String,
    pub command: Option<String>,
    pub dir: String,
}
