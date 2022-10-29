use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::canvas::{Canvas, Rectangle},
};
pub struct Tabss<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> Tabss<'a> {
    pub fn new() -> Tabss<'a> {
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

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: Tabss<'a>,
    pub ball: Rectangle,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            tabs: Tabss::new(),
            ball: Rectangle {
                x: 10.0,
                y: 30.0,
                width: 10.0,
                height: 10.0,
                color: Color::Yellow,
            },
        }
    }

    pub fn up(&mut self) {
        self.ball.y += 1 as f64;
    }

    pub fn down(&mut self) {
        self.ball.y -= 1 as f64;
    }
    pub fn left(&mut self) {
        self.ball.x -= 1 as f64;
    }
    pub fn right(&mut self) {
        self.ball.x += 1 as f64;
    }

    pub fn next_tab(&mut self) {
        self.tabs.next();
    }
}
