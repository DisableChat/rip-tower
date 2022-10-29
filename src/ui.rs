use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, BorderType, Borders, Paragraph, Tabs,
    },
    Frame, Terminal,
};

use crate::app::App;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let titles = app
        .tabs
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
        .select(app.tabs.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, chunks[1], app),
        1 => draw_second_tab(f, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, area: Rect, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default();

    //app.ball.x = app.ball.x + 1 as f64;
    let dots = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Goblins Incoming"),
        )
        .paint(|ctx| {
            ctx.draw(&app.ball);
        })
        .x_bounds([10.0, 110.0])
        .y_bounds([10.0, 110.0]);

    f.render_widget(dots, area);
}

fn draw_second_tab<B>(f: &mut Frame<B>, area: Rect)
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
                .style(Style::default().bg(Color::LightBlue).fg(Color::White))
                .title("Get Rekt")
                .border_type(BorderType::Plain),
        );
    f.render_widget(rip_chat, chunks[0]);
    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
