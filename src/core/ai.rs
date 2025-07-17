#[cfg(feature = "ai")]
use anyhow::Result;
#[cfg(feature = "ai")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "ai")]
use std::collections::HashMap;

#[cfg(feature = "ai")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAssistant {
    enabled: bool,
    api_key: Option<String>,
    suggestions_cache: HashMap<String, Vec<String>>,
}

#[cfg(feature = "ai")]
impl AIAssistant {
    pub fn new() -> Self {
        Self {
            enabled: false,
            api_key: None,
            suggestions_cache: HashMap::new(),
        }
    }
    
    pub fn enable(&mut self, api_key: String) {
        self.api_key = Some(api_key);
        self.enabled = true;
    }
    
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    pub async fn get_command_suggestions(&mut self, context: &str) -> Result<Vec<String>> {
        if !self.enabled {
            return Ok(vec![]);
        }
        
        // Check cache first
        if let Some(cached) = self.suggestions_cache.get(context) {
            return Ok(cached.clone());
        }
        
        // Mock AI suggestions for now
        let suggestions = vec![
            "ls -la".to_string(),
            "cd ..".to_string(),
            "git status".to_string(),
            "npm start".to_string(),
            "docker ps".to_string(),
        ];
        
        self.suggestions_cache.insert(context.to_string(), suggestions.clone());
        Ok(suggestions)
    }
    
    pub async fn explain_command(&self, command: &str) -> Result<String> {
        if !self.enabled {
            return Ok("AI assistant is disabled".to_string());
        }
        
        // Mock explanation for now
        let explanation = match command {
            "ls -la" => "Lists all files and directories in long format with hidden files",
            "cd .." => "Changes to the parent directory",
            "git status" => "Shows the status of files in the Git repository",
            _ => "Command explanation not available",
        };
        
        Ok(explanation.to_string())
    }
}

#[cfg(not(feature = "ai"))]
pub struct AIAssistant;

#[cfg(not(feature = "ai"))]
impl AIAssistant {
    pub fn new() -> Self {
        Self
    }
}