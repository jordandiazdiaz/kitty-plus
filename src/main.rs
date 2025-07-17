use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

mod core;
mod gpu;
mod ui;
mod config;
mod terminal;

use crate::config::Config;
use crate::terminal::Terminal;
use crate::ui::App;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let config = Config::load()?;
    let terminal = Arc::new(Mutex::new(Terminal::new(config.clone())?));
    
    let app = App::new(terminal, config).await?;
    app.run().await?;
    
    Ok(())
}