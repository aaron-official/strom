use crate::models::{AudioFile, ConversionStatus};
use ratatui::widgets::TableState;
use std::fs;
use std::path::Path;

pub enum AppScreen {
    List,
    Prompt,
    Converting,
}

pub struct App {
    pub files: Vec<AudioFile>,
    pub table_state: TableState,
    pub screen: AppScreen,
    pub should_quit: bool,
}

impl App {
    pub fn new(dir: &Path) -> std::io::Result<Self> {
        let mut files = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("m4b") {
                if let Some(file_name) = path.file_name() {
                    files.push(AudioFile {
                        filename: file_name.to_string_lossy().into_owned(),
                        path,
                        selected: false,
                        status: ConversionStatus::Ready,
                        duration_ms: 0,
                    });
                }
            }
        }

        // Sort files alphabetically to ensure deterministic order
        files.sort_by(|a, b| a.filename.cmp(&b.filename));

        let mut table_state = TableState::default();
        if !files.is_empty() {
            table_state.select(Some(0));
        }

        Ok(Self {
            files,
            table_state,
            screen: AppScreen::List,
            should_quit: false,
        })
    }

    pub fn next(&mut self) {
        if self.files.is_empty() { return; }
        let i = match self.table_state.selected() {
            Some(i) => if i >= self.files.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.files.is_empty() { return; }
        let i = match self.table_state.selected() {
            Some(i) => if i == 0 { self.files.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn toggle_selection(&mut self) {
        if let Some(i) = self.table_state.selected() {
            self.files[i].selected = !self.files[i].selected;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_navigation_empty() {
        let mut app = App {
            files: vec![],
            table_state: TableState::default(),
            screen: AppScreen::List,
            should_quit: false,
        };

        app.next();
        assert_eq!(app.table_state.selected(), None);
        app.previous();
        assert_eq!(app.table_state.selected(), None);
    }

    #[test]
    fn test_navigation_with_items() {
        let files = vec![
            AudioFile {
                filename: "1.m4b".to_string(),
                path: PathBuf::from("1.m4b"),
                selected: false,
                status: ConversionStatus::Ready,
                duration_ms: 0,
            },
            AudioFile {
                filename: "2.m4b".to_string(),
                path: PathBuf::from("2.m4b"),
                selected: false,
                status: ConversionStatus::Ready,
                duration_ms: 0,
            },
            AudioFile {
                filename: "3.m4b".to_string(),
                path: PathBuf::from("3.m4b"),
                selected: false,
                status: ConversionStatus::Ready,
                duration_ms: 0,
            },
        ];

        let mut table_state = TableState::default();
        table_state.select(Some(0));

        let mut app = App {
            files,
            table_state,
            screen: AppScreen::List,
            should_quit: false,
        };

        assert_eq!(app.table_state.selected(), Some(0));
        
        app.next();
        assert_eq!(app.table_state.selected(), Some(1));
        
        app.next();
        assert_eq!(app.table_state.selected(), Some(2));
        
        app.next();
        assert_eq!(app.table_state.selected(), Some(0));
        
        app.previous();
        assert_eq!(app.table_state.selected(), Some(2));
        
        app.previous();
        assert_eq!(app.table_state.selected(), Some(1));
    }

    #[test]
    fn test_toggle_selection() {
        let files = vec![
            AudioFile {
                filename: "1.m4b".to_string(),
                path: PathBuf::from("1.m4b"),
                selected: false,
                status: ConversionStatus::Ready,
                duration_ms: 0,
            },
        ];

        let mut table_state = TableState::default();
        table_state.select(Some(0));

        let mut app = App {
            files,
            table_state,
            screen: AppScreen::List,
            should_quit: false,
        };

        assert!(!app.files[0].selected);
        
        app.toggle_selection();
        assert!(app.files[0].selected);
        
        app.toggle_selection();
        assert!(!app.files[0].selected);
    }
}