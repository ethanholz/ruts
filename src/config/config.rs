use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RutConfig {
    pub workspaces: Option<Vec<WorkspaceConfig>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceConfig {
    pub name: String,
    pub windows: Option<Vec<Window>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub name: String,
    pub command: Option<String>,
    pub dir: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window() {
        let test_window = "
name: window
command: /bin/bash
dir: /home";
        let test_window_parse: Result<Window, serde_yaml::Error> =
            serde_yaml::from_str(test_window);
        assert!(test_window_parse.is_ok());
        let unwrapped = test_window_parse.unwrap();
        let window_val = Window {
            name: "window".to_string(),
            command: Some("/bin/bash".to_string()),
            dir: "/home".to_string(),
        };
        assert_eq!(unwrapped, window_val);
    }

    #[test]
    fn test_workspace_config() {
        let test = "
name: test_conf
windows: 
    - name: window1
      dir: /home
    - name: window2
      dir: /home
";
        let test_workspace_parse: Result<WorkspaceConfig, serde_yaml::Error> =
            serde_yaml::from_str(test);
        assert!(test_workspace_parse.is_ok());
        let workspace = test_workspace_parse.unwrap();
        let window1: Window = Window {
            name: "window1".to_string(),
            command: None,
            dir: "/home".to_string(),
        };
        let window2: Window = Window {
            name: "window2".to_string(),
            command: None,
            dir: "/home".to_string(),
        };
        let windows = Some(vec![window1, window2]);
        let workspace_compare = WorkspaceConfig {
            name: "test_conf".to_string(),
            windows,
        };
        assert_eq!(workspace.name, String::from("test_conf"));
        assert_eq!(workspace, workspace_compare);
    }
}
