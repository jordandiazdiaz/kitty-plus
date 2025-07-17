use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use vte::{Params, Parser, Perform};

use crate::config::Config;

pub struct Terminal {
    parser: Parser,
    state: TerminalState,
    config: Config,
}

#[derive(Default)]
pub struct TerminalState {
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub rows: usize,
    pub cols: usize,
    pub buffer: Vec<Vec<Cell>>,
    pub alt_buffer: Vec<Vec<Cell>>,
    pub using_alt_buffer: bool,
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
}

#[derive(Clone, Default)]
pub struct Cell {
    pub c: char,
    pub fg: Color,
    pub bg: Color,
    pub attrs: Attributes,
}

#[derive(Clone, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Copy, Default)]
pub struct Attributes {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub blink: bool,
}

pub struct Tab {
    pub title: String,
    pub has_activity: bool,
    pub process_id: u32,
}

impl Terminal {
    pub fn new(config: Config) -> Result<Self> {
        let mut state = TerminalState::default();
        state.rows = 24;
        state.cols = 80;
        state.buffer = vec![vec![Cell::default(); state.cols]; state.rows];
        state.alt_buffer = vec![vec![Cell::default(); state.cols]; state.rows];
        
        state.tabs.push(Tab {
            title: "Terminal".to_string(),
            has_activity: false,
            process_id: 0,
        });
        
        Ok(Self {
            parser: Parser::new(),
            state,
            config,
        })
    }
    
    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.state.rows = rows;
        self.state.cols = cols;
        self.state.buffer.resize(rows, vec![Cell::default(); cols]);
        for row in &mut self.state.buffer {
            row.resize(cols, Cell::default());
        }
    }
    
    pub fn process_input(&mut self, data: &[u8]) {
        for byte in data {
            self.parser.advance(self, *byte);
        }
    }
    
    pub fn create_new_tab(&mut self, title: String) {
        self.state.tabs.push(Tab {
            title,
            has_activity: false,
            process_id: 0,
        });
    }
    
    pub fn mark_tab_activity(&mut self, tab_index: usize) {
        if tab_index != self.state.active_tab && tab_index < self.state.tabs.len() {
            self.state.tabs[tab_index].has_activity = true;
        }
    }
    
    pub fn switch_tab(&mut self, tab_index: usize) {
        if tab_index < self.state.tabs.len() {
            self.state.active_tab = tab_index;
            self.state.tabs[tab_index].has_activity = false;
        }
    }
}

impl Perform for Terminal {
    fn print(&mut self, c: char) {
        let current_buffer = if self.state.using_alt_buffer {
            &mut self.state.alt_buffer
        } else {
            &mut self.state.buffer
        };
        
        if self.state.cursor_y < self.state.rows && self.state.cursor_x < self.state.cols {
            current_buffer[self.state.cursor_y][self.state.cursor_x].c = c;
            self.state.cursor_x += 1;
            
            if self.state.cursor_x >= self.state.cols {
                self.state.cursor_x = 0;
                self.state.cursor_y += 1;
            }
        }
    }
    
    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.state.cursor_x = 0;
                self.state.cursor_y += 1;
            }
            b'\r' => {
                self.state.cursor_x = 0;
            }
            b'\t' => {
                self.state.cursor_x = (self.state.cursor_x + 8) & !7;
            }
            _ => {}
        }
    }
    
    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {}
    fn put(&mut self, _byte: u8) {}
    fn unhook(&mut self) {}
    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {}
    fn csi_dispatch(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {}
    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {}
}