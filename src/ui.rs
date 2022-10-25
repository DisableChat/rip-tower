use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::app::App;

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

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, menu: &mut Tabss) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.title.to_string()),
        )
        .select(menu.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);
    match menu.index {
        0 => draw_first_tab(f, menu, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, menu: &mut Tabss, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(area);
    let rip_chat = Paragraph::new("Big Ol Rips in Amish")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::LightRed).fg(Color::White))
                .title("Get Rekt")
                .border_type(BorderType::Plain),
        );
    f.render_widget(rip_chat, chunks[0]);
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
