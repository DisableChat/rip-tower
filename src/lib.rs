// lib.rs
use std::cell::RefCell;
use std::rc::Rc;

use std::io::stdout;

use app::App;
use eyre::Result;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::ui;

pub mod app;

pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    // Configure Crossterm backend for tui
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let app = app.borrow();
        // Render
        terminal.draw(|rect| ui::draw(rect, &app))?;
        // TODO handle inputs here
    }

    // Restore the terminal and close applicationi
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}