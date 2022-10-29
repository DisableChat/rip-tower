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

use crate::events;
use crate::key::Key;
use crate::ui::ui;
use crate::{app::App, events::Event};

#[derive(PartialEq)]
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
    pub reset: bool,
    pub vector_field: Vec<Vec<Vdirection>>,
}

impl Goblin {
    pub fn new(_position: Position) -> Self {
        Goblin {
            position: Position {
                x: (10.0),
                y: (10.0),
            },
            side: Color::Green,
            reset: false,
            vector_field: vec![],
        }
    }

    pub fn attack(&mut self) {
        let end_location = Position {
            x: (100.0),
            y: (100.0),
        };
        if self.position != end_location {
            self.position.x += 1.0;
            self.position.y += 1.0;
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
    terminal.hide_cursor()?;

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
        let res = match events.next() {
            Ok(Event::Input(key)) => app.handle_key_action(key),
            Ok(Event::Tick) => app.handle_tick(),
            Err(err) => println!("Oops, something wrong happen: {:?}", err),
        };
    }

    Ok(())
}
