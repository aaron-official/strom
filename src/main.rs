pub mod app;
pub mod audio;
pub mod models;

use app::{App, AppScreen};
use crate::audio::{get_duration_ms, get_output_path, convert_with_progress};
use crate::models::ConversionStatus;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Row, Table},
    Terminal,
};
use std::{env, io, sync::{Arc, Mutex}, time::Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let current_dir = env::current_dir()?;
    let app = Arc::new(Mutex::new(App::new(&current_dir)?));

    loop {
        {
            let mut app = app.lock().unwrap();
            if app.should_quit {
                break;
            }
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .constraints([Constraint::Percentage(100)])
                    .split(f.size());

                match app.screen {
                    AppScreen::List => {
                        let rows: Vec<Row> = app.files.iter().map(|item| {
                            let style = if item.selected { Style::default().fg(Color::Yellow) } else { Style::default() };
                            let status_str = match &item.status {
                                ConversionStatus::Ready => "Ready".to_string(),
                                ConversionStatus::ExtractingMetadata => "Extracting...".to_string(),
                                ConversionStatus::Converting(p) => format!("[Converting {:.0}%]", p * 100.0),
                                ConversionStatus::Done => "Done".to_string(),
                                ConversionStatus::Error(_) => "Error".to_string(),
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
                        let chunks = Layout::default()
                            .direction(ratatui::layout::Direction::Vertical)
                            .constraints([
                                Constraint::Length(3), // Progress bar
                                Constraint::Min(0),    // Message
                            ])
                            .split(f.size());

                        let progress = app.get_overall_progress();
                        let gauge = Gauge::default()
                            .block(Block::default().title(" Overall Progress ").borders(Borders::ALL))
                            .gauge_style(Style::default().fg(Color::Yellow))
                            .percent((progress * 100.0) as u16);
                        f.render_widget(gauge, chunks[0]);

                        let b = Block::default().title(" Converting... Please wait or press q to abort ").borders(Borders::ALL);
                        f.render_widget(b, chunks[1]);
                    }
                }
            })?;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let mut app_lock = app.lock().unwrap();
                match app_lock.screen {
                    AppScreen::List => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down | KeyCode::Char('j') => app_lock.next(),
                        KeyCode::Up | KeyCode::Char('k') => app_lock.previous(),
                        KeyCode::Char(' ') => app_lock.toggle_selection(),
                        KeyCode::Enter => {
                            if app_lock.files.iter().any(|f| f.selected) {
                                app_lock.screen = AppScreen::Prompt;
                            }
                        }
                        _ => {}
                    },
                    AppScreen::Prompt => match key.code {
                        KeyCode::Char('y') => {
                            app_lock.screen = AppScreen::Converting;
                            let selected_indices: Vec<usize> = app_lock.files.iter()
                                .enumerate()
                                .filter(|(_, f)| f.selected)
                                .map(|(i, _)| i)
                                .collect();

                            for index in selected_indices {
                                let app_clone = Arc::clone(&app);
                                tokio::spawn(async move {
                                    let (file_path, mut duration_ms) = {
                                        let app = app_clone.lock().unwrap();
                                        let file = &app.files[index];
                                        (file.path.clone(), file.duration_ms)
                                    };

                                    if duration_ms == 0 {
                                        {
                                            let mut app = app_clone.lock().unwrap();
                                            app.update_file_status(index, ConversionStatus::ExtractingMetadata);
                                        }
                                        match get_duration_ms(&file_path).await {
                                            Ok(d) => {
                                                duration_ms = d;
                                                let mut app = app_clone.lock().unwrap();
                                                app.update_file_duration(index, d);
                                            }
                                            Err(e) => {
                                                let mut app = app_clone.lock().unwrap();
                                                app.update_file_status(index, ConversionStatus::Error(e.to_string()));
                                                return;
                                            }
                                        }
                                    }

                                    let output_dir = match get_output_path(&file_path, false) {
                                        Ok(p) => p,
                                        Err(e) => {
                                            let mut app = app_clone.lock().unwrap();
                                            app.update_file_status(index, ConversionStatus::Error(e.to_string()));
                                            return;
                                        }
                                    };
                                    let file_stem = file_path.file_stem().unwrap().to_string_lossy();
                                    let output_path = output_dir.join(format!("{}.mp3", file_stem));

                                    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
                                    
                                    let app_clone_inner = Arc::clone(&app_clone);
                                    let progress_task = tokio::spawn(async move {
                                        while let Some(progress) = rx.recv().await {
                                            let mut app = app_clone_inner.lock().unwrap();
                                            app.update_file_status(index, ConversionStatus::Converting(progress));
                                        }
                                    });

                                    match convert_with_progress(&file_path, &output_path, duration_ms, tx).await {
                                        Ok(_) => {
                                            let mut app = app_clone.lock().unwrap();
                                            app.update_file_status(index, ConversionStatus::Done);
                                        }
                                        Err(e) => {
                                            let mut app = app_clone.lock().unwrap();
                                            app.update_file_status(index, ConversionStatus::Error(e.to_string()));
                                        }
                                    }
                                    let _ = progress_task.await;
                                });
                            }
                        }
                        KeyCode::Char('n') => app_lock.screen = AppScreen::List,
                        KeyCode::Char('q') => break,
                        _ => {}
                    },
                    AppScreen::Converting => match key.code {
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
