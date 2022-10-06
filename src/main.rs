use eyre::Result;

mod events;
mod key;
mod ui;
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
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

use key::Key;
struct Tabss<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> Tabss<'a> {
    fn new() -> Tabss<'a> {
        Tabss {
            titles: vec!["Home", "Stats", "Help"],
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

// main.rs
fn main() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = events::Events::new();
    /*
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(events::Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(events::Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
    */

    loop {
        terminal.draw(ui)?;
        if let events::Event::Input(event) = events.next()? {
            match event {
                //KeyCode::Char('c') | KeyCode::Char('q') => {break;}
                Key::Ctrl('c') | Key::Char('q') => {
                    break;
                }
                Key::Up => {
                    println!("Base 10 repr:               {}", 69420);
                }
                _ => {}
            }
        }
        /*     match rx.recv()? {
                events::Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        //disable_raw_mode()?;
                        //terminal.show_cursor()?;
                        //break;
                        break;
                    }
                    //KeyCode::Char('h') => tabs.next(),
                    //KeyCode::Char('p') => active_menu_item = MenuItem::Pets,
                    //KeyCode::Char('d') => {
                    //    remove_pet_at_index(&mut pet_list_state).expect("can remove pet");
                    //}
                    //KeyCode::Down => {
                    //}
                    _ => {}
                },
                events::Event::Tick => {}
            }
        }
        */
    }
    //thread::sleep(Duration::from_millis(5000));
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
} // Terminal initialization for UI

pub fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(4),
                Constraint::Min(4),
                Constraint::Length(4),
            ]
            .as_ref(),
        )
        .split(f.size());

    let menu = Tabss::new();

    let titles = menu
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(menu.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);

    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);

    f.render_widget(block, chunks[2]);

    let rip_chat = Paragraph::new("Fucking Rip")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::LightRed).fg(Color::White))
                .title("Get Rekt")
                .border_type(BorderType::Plain),
        );

    f.render_widget(rip_chat, chunks[3]);
}
