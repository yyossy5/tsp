use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Complete structure of layout configuration file
#[derive(Debug, Deserialize, Serialize)]
pub struct LayoutConfig {
    pub workspace: WorkspaceConfig,
    pub panes: Vec<PaneConfig>,
}

/// Workspace configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceConfig {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub directory: String,
}

/// Individual pane configuration
#[derive(Debug, Deserialize, Serialize)]
pub struct PaneConfig {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default)]
    pub split: Option<String>, // "horizontal" or "vertical"
    #[serde(default)]
    pub commands: Vec<String>,
    #[serde(default)]
    pub focus: bool,
}

impl LayoutConfig {
    /// Load layout configuration from YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: LayoutConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Apply layout to tmux
    pub fn apply_to_tmux(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if directory exists
        let target_dir = Path::new(&self.workspace.directory);
        if !target_dir.exists() {
            return Err(format!("Directory '{}' does not exist", self.workspace.directory).into());
        }

        // Get absolute path
        let absolute_path = target_dir.canonicalize()?;
        let path_str = absolute_path.to_string_lossy();

        // Move current pane to target directory
        Command::new("tmux")
            .args(["send-keys", &format!("cd '{}'", path_str), "Enter"])
            .status()?;

        // Create additional panes
        self.create_panes(&path_str)?;

        // Apply layout
        self.arrange_layout()?;

        // Execute commands in each pane
        self.execute_commands()?;

        // Set focus
        self.set_focus()?;

        println!("Applied layout '{}' in directory: {}", self.workspace.name, path_str);
        Ok(())
    }

    /// Create panes
    fn create_panes(&self, base_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        for (index, pane) in self.panes.iter().enumerate().skip(1) {
            let split_direction = match pane.split.as_deref() {
                Some("horizontal") => "-v",
                Some("vertical") => "-h",
                _ => if index % 2 == 1 { "-h" } else { "-v" }, // Default alternating split
            };

            Command::new("tmux")
                .args(["split-window", split_direction, "-c", base_path])
                .status()
                .map_err(|e| format!("Failed to create pane {}: {}", index + 1, e))?;
        }
        Ok(())
    }

    /// Arrange layout
    fn arrange_layout(&self) -> Result<(), Box<dyn std::error::Error>> {
        Command::new("tmux")
            .args(["select-layout", "tiled"])
            .status()
            .map_err(|e| format!("Failed to arrange panes: {}", e))?;
        Ok(())
    }

    /// Execute commands in each pane
    fn execute_commands(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (index, pane) in self.panes.iter().enumerate() {
            // Select pane
            Command::new("tmux")
                .args(["select-pane", "-t", &index.to_string()])
                .status()?;

            // Execute commands
            for command in &pane.commands {
                Command::new("tmux")
                    .args(["send-keys", command, "Enter"])
                    .status()
                    .map_err(|e| format!("Failed to execute command '{}' in pane {}: {}", command, index, e))?;
            }
        }
        Ok(())
    }

    /// Set focus
    fn set_focus(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (index, pane) in self.panes.iter().enumerate() {
            if pane.focus {
                Command::new("tmux")
                    .args(["select-pane", "-t", &index.to_string()])
                    .status()?;
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_parsing() {
        let yaml_content = r#"
workspace:
  name: "test-layout"
  description: "Test layout for development"
  directory: "/tmp"

panes:
  - id: "editor"
    commands:
      - "echo 'Editor pane'"
    focus: true
  - id: "terminal"
    split: "horizontal"
    commands:
      - "echo 'Terminal pane'"
"#;

        let config: LayoutConfig = serde_yaml::from_str(yaml_content).unwrap();
        assert_eq!(config.workspace.name, "test-layout");
        assert_eq!(config.panes.len(), 2);
        assert!(config.panes[0].focus);
    }
}