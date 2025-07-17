use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::config::Config;
use crate::terminal::Terminal;
use crate::gpu::GpuRenderer;
use super::command_palette::CommandPalette;

pub struct App {
    terminal: Arc<Mutex<Terminal>>,
    config: Config,
    command_palette: CommandPalette,
    gpu_renderer: Option<GpuRenderer>,
}

impl App {
    pub async fn new(terminal: Arc<Mutex<Terminal>>, config: Config) -> Result<Self> {
        Ok(Self {
            terminal,
            config: config.clone(),
            command_palette: CommandPalette::new(config.clone()),
            gpu_renderer: None,
        })
    }
    
    pub async fn run(mut self) -> Result<()> {
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title("Kitty Plus - Modern Terminal")
            .with_inner_size(winit::dpi::LogicalSize::new(1024, 768))
            .build(&event_loop)?;
        
        if self.config.performance.gpu_acceleration {
            self.gpu_renderer = Some(GpuRenderer::new(&window).await?);
        }
        
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { input, .. },
                    ..
                } => {
                    self.handle_keyboard_input(input);
                }
                
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    ..
                } => {
                    if let Some(renderer) = &mut self.gpu_renderer {
                        renderer.resize(size.width, size.height);
                    }
                }
                
                Event::RedrawRequested(_) => {
                    if let Some(renderer) = &mut self.gpu_renderer {
                        let terminal = self.terminal.clone();
                        tokio::spawn(async move {
                            let term = terminal.lock().await;
                            // renderer.render(&term);
                        });
                    }
                }
                
                _ => {}
            }
        });
    }
    
    fn handle_keyboard_input(&mut self, _input: winit::event::KeyboardInput) {
        // Handle keyboard shortcuts like Ctrl+Shift+P for command palette
        if self.config.features.command_palette {
            // self.command_palette.toggle();
        }
    }
}