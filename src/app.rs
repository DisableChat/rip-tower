use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::canvas::{Canvas, Rectangle},
};

use crate::key::Key;
pub struct Tabs<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> Tabs<'a> {
    pub fn new() -> Tabs<'a> {
        Tabs {
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

pub struct App<'a> {
    pub quit: bool,
    pub title: &'a str,
    pub tabs: Tabs<'a>,
    pub ball: Rectangle,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            quit: false,
            title,
            tabs: Tabs::new(),
            ball: Rectangle {
                x: 10.0,
                y: 30.0,
                width: 10.0,
                height: 10.0,
                color: Color::Green,
            },
        }
    }

    pub fn handle_key_action(&mut self, val: Key) {
        match val {
            Key::Ctrl('c') | Key::Char('q') => {
                self.quit = true;
            }
            Key::Tab => {
                self.tabs.next();
            }
            Key::Up => {
                self.ball.y += 1 as f64;
            }
            Key::Down => {
                self.ball.y -= 1 as f64;
            }
            Key::Left => {
                self.ball.x -= 1 as f64;
            }
            Key::Right => {
                self.ball.x += 1 as f64;
            }
            _ => {}
        }
    }
}
