use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecorder {
    recording: bool,
    start_time: Option<u64>,
    events: Vec<SessionEvent>,
    output_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEvent {
    pub timestamp: u64,
    pub event_type: EventType,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Input,
    Output,
    Resize { width: u32, height: u32 },
    Command(String),
}

impl SessionRecorder {
    pub fn new() -> Self {
        Self {
            recording: false,
            start_time: None,
            events: Vec::new(),
            output_file: None,
        }
    }
    
    pub fn start_recording(&mut self, output_file: Option<String>) -> Result<()> {
        self.recording = true;
        self.start_time = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_millis() as u64,
        );
        self.output_file = output_file;
        self.events.clear();
        Ok(())
    }
    
    pub fn stop_recording(&mut self) -> Result<()> {
        self.recording = false;
        
        if let Some(file_path) = &self.output_file {
            self.save_to_file(file_path)?;
        }
        
        Ok(())
    }
    
    pub fn record_input(&mut self, data: &[u8]) -> Result<()> {
        if !self.recording {
            return Ok(());
        }
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis() as u64;
        
        self.events.push(SessionEvent {
            timestamp,
            event_type: EventType::Input,
            data: data.to_vec(),
        });
        
        Ok(())
    }
    
    pub fn record_output(&mut self, data: &[u8]) -> Result<()> {
        if !self.recording {
            return Ok(());
        }
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis() as u64;
        
        self.events.push(SessionEvent {
            timestamp,
            event_type: EventType::Output,
            data: data.to_vec(),
        });
        
        Ok(())
    }
    
    pub fn record_resize(&mut self, width: u32, height: u32) -> Result<()> {
        if !self.recording {
            return Ok(());
        }
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis() as u64;
        
        self.events.push(SessionEvent {
            timestamp,
            event_type: EventType::Resize { width, height },
            data: vec![],
        });
        
        Ok(())
    }
    
    pub fn record_command(&mut self, command: String) -> Result<()> {
        if !self.recording {
            return Ok(());
        }
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis() as u64;
        
        self.events.push(SessionEvent {
            timestamp,
            event_type: EventType::Command(command),
            data: vec![],
        });
        
        Ok(())
    }
    
    pub fn is_recording(&self) -> bool {
        self.recording
    }
    
    pub fn get_events(&self) -> &[SessionEvent] {
        &self.events
    }
    
    fn save_to_file(&self, file_path: &str) -> Result<()> {
        let mut file = std::fs::File::create(file_path)?;
        
        // Write in asciinema format
        let header = serde_json::json!({
            "version": 2,
            "width": 80,
            "height": 24,
            "timestamp": self.start_time.unwrap_or(0),
            "env": {
                "SHELL": "/bin/bash",
                "TERM": "xterm-256color"
            }
        });
        
        writeln!(file, "{}", header)?;
        
        let start_time = self.start_time.unwrap_or(0);
        
        for event in &self.events {
            let relative_time = (event.timestamp - start_time) as f64 / 1000.0;
            
            match &event.event_type {
                EventType::Input => {
                    let data = String::from_utf8_lossy(&event.data);
                    let entry = serde_json::json!([relative_time, "i", data]);
                    writeln!(file, "{}", entry)?;
                }
                EventType::Output => {
                    let data = String::from_utf8_lossy(&event.data);
                    let entry = serde_json::json!([relative_time, "o", data]);
                    writeln!(file, "{}", entry)?;
                }
                EventType::Resize { width, height } => {
                    let entry = serde_json::json!([relative_time, "r", format!("{}x{}", width, height)]);
                    writeln!(file, "{}", entry)?;
                }
                EventType::Command(cmd) => {
                    let entry = serde_json::json!([relative_time, "c", cmd]);
                    writeln!(file, "{}", entry)?;
                }
            }
        }
        
        Ok(())
    }
}