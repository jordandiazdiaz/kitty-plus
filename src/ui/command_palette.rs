use crate::config::Config;

pub struct CommandPalette {
    visible: bool,
    query: String,
    commands: Vec<Command>,
    selected_index: usize,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub keybinding: Option<String>,
    pub action: CommandAction,
}

#[derive(Debug, Clone)]
pub enum CommandAction {
    NewTab,
    CloseTab,
    SwitchTab(usize),
    ToggleFullscreen,
    IncreaseFont,
    DecreaseFont,
    ResetFont,
    ToggleAISuggestions,
    StartRecording,
    StopRecording,
    OpenSettings,
    ShowHelp,
    Custom(String),
}

impl CommandPalette {
    pub fn new(config: Config) -> Self {
        let mut commands = vec![
            Command {
                name: "New Tab".to_string(),
                description: "Create a new terminal tab".to_string(),
                keybinding: Some("Ctrl+Shift+N".to_string()),
                action: CommandAction::NewTab,
            },
            Command {
                name: "Close Tab".to_string(),
                description: "Close the current tab".to_string(),
                keybinding: Some("Ctrl+Shift+W".to_string()),
                action: CommandAction::CloseTab,
            },
            Command {
                name: "Toggle Fullscreen".to_string(),
                description: "Toggle fullscreen mode".to_string(),
                keybinding: Some("F11".to_string()),
                action: CommandAction::ToggleFullscreen,
            },
            Command {
                name: "Increase Font Size".to_string(),
                description: "Make text larger".to_string(),
                keybinding: Some("Ctrl+=".to_string()),
                action: CommandAction::IncreaseFont,
            },
            Command {
                name: "Decrease Font Size".to_string(),
                description: "Make text smaller".to_string(),
                keybinding: Some("Ctrl+-".to_string()),
                action: CommandAction::DecreaseFont,
            },
            Command {
                name: "Reset Font Size".to_string(),
                description: "Reset font to default size".to_string(),
                keybinding: Some("Ctrl+0".to_string()),
                action: CommandAction::ResetFont,
            },
            Command {
                name: "Settings".to_string(),
                description: "Open settings".to_string(),
                keybinding: Some("Ctrl+,".to_string()),
                action: CommandAction::OpenSettings,
            },
            Command {
                name: "Help".to_string(),
                description: "Show help documentation".to_string(),
                keybinding: Some("F1".to_string()),
                action: CommandAction::ShowHelp,
            },
        ];
        
        // Add AI features if enabled
        if config.features.ai_suggestions {
            commands.push(Command {
                name: "Toggle AI Suggestions".to_string(),
                description: "Enable/disable AI command suggestions".to_string(),
                keybinding: Some("Ctrl+Shift+A".to_string()),
                action: CommandAction::ToggleAISuggestions,
            });
        }
        
        // Add session recording if enabled
        if config.features.session_recording {
            commands.extend([
                Command {
                    name: "Start Recording".to_string(),
                    description: "Start recording terminal session".to_string(),
                    keybinding: Some("Ctrl+Shift+R".to_string()),
                    action: CommandAction::StartRecording,
                },
                Command {
                    name: "Stop Recording".to_string(),
                    description: "Stop recording terminal session".to_string(),
                    keybinding: Some("Ctrl+Shift+S".to_string()),
                    action: CommandAction::StopRecording,
                },
            ]);
        }
        
        Self {
            visible: false,
            query: String::new(),
            commands,
            selected_index: 0,
        }
    }
    
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        if self.visible {
            self.query.clear();
            self.selected_index = 0;
        }
    }
    
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    pub fn update_query(&mut self, query: String) {
        self.query = query;
        self.selected_index = 0;
    }
    
    pub fn get_filtered_commands(&self) -> Vec<&Command> {
        if self.query.is_empty() {
            self.commands.iter().collect()
        } else {
            self.commands
                .iter()
                .filter(|cmd| {
                    cmd.name.to_lowercase().contains(&self.query.to_lowercase())
                        || cmd.description.to_lowercase().contains(&self.query.to_lowercase())
                })
                .collect()
        }
    }
    
    pub fn select_next(&mut self) {
        let filtered = self.get_filtered_commands();
        if !filtered.is_empty() {
            self.selected_index = (self.selected_index + 1) % filtered.len();
        }
    }
    
    pub fn select_previous(&mut self) {
        let filtered = self.get_filtered_commands();
        if !filtered.is_empty() {
            self.selected_index = if self.selected_index > 0 {
                self.selected_index - 1
            } else {
                filtered.len() - 1
            };
        }
    }
    
    pub fn get_selected_command(&self) -> Option<&Command> {
        let filtered = self.get_filtered_commands();
        filtered.get(self.selected_index).copied()
    }
    
    pub fn execute_selected(&mut self) -> Option<CommandAction> {
        if let Some(command) = self.get_selected_command() {
            self.visible = false;
            Some(command.action.clone())
        } else {
            None
        }
    }
}