pub mod app;
pub mod audio;
pub mod models;

use app::{App, AppScreen};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Terminal,
};
use std::{env, io};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let current_dir = env::current_dir()?;
    let mut app = App::new(&current_dir)?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .constraints([Constraint::Percentage(100)])
                .split(f.size());

            match app.screen {
                AppScreen::List => {
                    let rows: Vec<Row> = app.files.iter().map(|item| {
                        let style = if item.selected { Style::default().fg(Color::Yellow) } else { Style::default() };
                        let status_str = match &item.status {
                            models::ConversionStatus::Ready => "Ready".to_string(),
                            models::ConversionStatus::ExtractingMetadata => "Extracting...".to_string(),
                            models::ConversionStatus::Converting(p) => format!("Converting {:.0}%", p * 100.0),
                            models::ConversionStatus::Done => "Done".to_string(),
                            models::ConversionStatus::Error(_) => "Error".to_string(),
                        };
                        let sel_marker = if item.selected { "[x]" } else { "[ ]" };
                        Row::new(vec![sel_marker.to_string(), item.filename.clone(), status_str.to_string()])
                            .style(style)
                    }).collect();

                    let table = Table::new(rows, [Constraint::Length(5), Constraint::Min(20), Constraint::Length(15)])
                        .block(Block::default().title(" Strom - m4b to mp3 (Space: Select, Enter: Convert, q: Quit) ").borders(Borders::ALL))
                        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

                    f.render_stateful_widget(table, chunks[0], &mut app.table_state);
                }
                AppScreen::Prompt => {
                    let b = Block::default().title(" Proceed? (y/n) ").borders(Borders::ALL);
                    f.render_widget(b, chunks[0]);
                }
                AppScreen::Converting => {
                    let b = Block::default().title(" Converting... Please wait or press q to abort ").borders(Borders::ALL);
                    f.render_widget(b, chunks[0]);
                }
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match app.screen {
                AppScreen::List => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Char(' ') => app.toggle_selection(),
                    KeyCode::Enter => {
                        if app.files.iter().any(|f| f.selected) {
                            app.screen = AppScreen::Prompt;
                        }
                    }
                    _ => {}
                },
                AppScreen::Prompt => match key.code {
                    KeyCode::Char('y') => app.screen = AppScreen::Converting,
                    KeyCode::Char('n') => app.screen = AppScreen::List,
                    KeyCode::Char('q') => break,
                    _ => {}
                },
                AppScreen::Converting => match key.code {
                    KeyCode::Char('q') => break, // Simplified abort for now
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
