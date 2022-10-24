use eyre::Result;

mod events;
mod key;
mod ui;

mod app;
mod rip_tower;

use crate::rip_tower::run;

// main.rs
fn main() -> Result<()> {
    run()?;
    Ok(())
} // Terminal initialization for UI
