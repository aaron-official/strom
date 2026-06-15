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
        if self.files.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.files.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.files.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.files.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn toggle_selection(&mut self) {
        if let Some(i) = self.table_state.selected() {
            self.files[i].selected = !self.files[i].selected;
        }
    }

    pub fn get_overall_progress(&self) -> f64 {
        let selected_files: Vec<_> = self.files.iter().filter(|f| f.selected).collect();
        if selected_files.is_empty() {
            return 0.0;
        }

        let total_progress: f64 = selected_files
            .iter()
            .map(|f| match f.status {
                ConversionStatus::Done => 1.0,
                ConversionStatus::Converting(p) => p,
                _ => 0.0,
            })
            .sum();

        total_progress / selected_files.len() as f64
    }

    pub fn update_file_status(&mut self, index: usize, status: ConversionStatus) {
        if let Some(file) = self.files.get_mut(index) {
            file.status = status;
        }
    }

    pub fn update_file_duration(&mut self, index: usize, duration_ms: u64) {
        if let Some(file) = self.files.get_mut(index) {
            file.duration_ms = duration_ms;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_overall_progress() {
        let files = vec![
            AudioFile {
                filename: "1.m4b".to_string(),
                path: PathBuf::from("1.m4b"),
                selected: true,
                status: ConversionStatus::Done,
                duration_ms: 0,
            },
            AudioFile {
                filename: "2.m4b".to_string(),
                path: PathBuf::from("2.m4b"),
                selected: true,
                status: ConversionStatus::Converting(0.5),
                duration_ms: 0,
            },
            AudioFile {
                filename: "3.m4b".to_string(),
                path: PathBuf::from("3.m4b"),
                selected: false,
                status: ConversionStatus::Converting(0.9),
                duration_ms: 0,
            },
            AudioFile {
                filename: "4.m4b".to_string(),
                path: PathBuf::from("4.m4b"),
                selected: true,
                status: ConversionStatus::Ready,
                duration_ms: 0,
            },
        ];

        let app = App {
            files,
            table_state: TableState::default(),
            screen: AppScreen::List,
            should_quit: false,
        };

        // Selected files are 1 (1.0), 2 (0.5), 4 (0.0).
        // Total = 1.5. Count = 3. Average = 0.5.
        assert_eq!(app.get_overall_progress(), 0.5);
    }

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
        let files = vec![AudioFile {
            filename: "1.m4b".to_string(),
            path: PathBuf::from("1.m4b"),
            selected: false,
            status: ConversionStatus::Ready,
            duration_ms: 0,
        }];

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

    #[test]
    fn test_update_status_and_duration() {
        let files = vec![AudioFile {
            filename: "1.m4b".to_string(),
            path: PathBuf::from("1.m4b"),
            selected: false,
            status: ConversionStatus::Ready,
            duration_ms: 0,
        }];

        let mut app = App {
            files,
            table_state: TableState::default(),
            screen: AppScreen::List,
            should_quit: false,
        };

        app.update_file_status(0, ConversionStatus::Converting(0.5));
        assert_eq!(app.files[0].status, ConversionStatus::Converting(0.5));

        app.update_file_duration(0, 1000);
        assert_eq!(app.files[0].duration_ms, 1000);
    }
}
