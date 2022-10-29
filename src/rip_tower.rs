use eyre::{private::kind::TraitKind, Result};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{canvas::Points, Block, BorderType, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::app::App;
use crate::events;
use crate::key::Key;
use crate::ui::ui;

pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub enum Vdirection {
    Left,
    Right,
    Up,
    Down,
}

pub struct Goblin {
    pub position: Position,
    pub side: Color,
    pub vector_field: Vec<Vec<Vdirection>>,
}

impl Goblin {
    pub fn new(position: Position) -> Self {
        Goblin {
            position: Position {
                x: (20.0),
                y: (20.),
            },
            side: Color::Green,
            vector_field: vec![],
        }
    }
}

pub fn run() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create App and run it
    let mut app = App::new("Rip Tower Swag");
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let events = events::Events::new();
    loop {
        if app.quit {
            break;
        }

        // Render
        terminal.draw(|f| ui(f, app))?;

        // Handle Inputs
        if let events::Event::Input(event) = events.next()? {
            app.handle_key_action(event);
        }
    }

    Ok(())
}
