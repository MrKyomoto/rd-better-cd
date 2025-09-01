use std::error::Error;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    DefaultTerminal,
};

use crate::{
    components::{filter_hidden_files, get_current_dir, get_files, get_parent_dir, File, FileType},
    ui::render_ui,
};

pub struct App {
    pub current_dir: String,
    pub files: Vec<File>,
    pub index: usize,
    /// NOTE: index of the current sub dir
    pub sub_files: Vec<File>, // NOTE: sub dir's files
    pub filter_hidden_file: bool,
    pub exit: bool,
    pub output: bool,
}

impl App {
    pub fn new() -> Self {
        let current_dir = get_current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap_or(String::from("."));
        let files = get_files(&current_dir).unwrap();
        let sub_files = if !files.is_empty() {
            match files[0].file_type {
                // MEMO: due to the get_current_dir won't append '/' at the ending of the path so
                // we have to add it mannually and THIS IS THE BUG WHICH COSTS MUCH TIME :(
                FileType::Dir => get_files(&(current_dir.clone() + "/" + &files[0].name)).unwrap(),
                FileType::File => Vec::new(),
            }
        } else {
            Vec::new()
        };
        App {
            current_dir: current_dir,
            files: files,
            index: 0,
            sub_files: sub_files,
            filter_hidden_file: false,
            exit: false,
            output: true,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
        while !self.exit {
            terminal.draw(|f| render_ui(f, &self).unwrap())?;
            self.handle_event()?;
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_event(&mut self) -> Result<(), Box<dyn Error>> {
        match event::read()? {
            event::Event::Key(key_event) => {
                self.handle_key_evnet(&key_event)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// NOTE: There are several keybinds to control the app:
    /// <ESC> -> back to the parent dir
    /// <Enter> -> into the sub dir
    /// <Up|Down> -> switch the pointted sub dir | here we have vim version as well(<j|k>)
    /// <Space> -> select the pointted sub dir as the finnal working dir
    /// <h> -> fillter hidden files (start with '.')
    fn handle_key_evnet(&mut self, key_event: &KeyEvent) -> Result<(), Box<dyn Error>> {
        if key_event.kind == KeyEventKind::Release {
            return Ok(());
        }

        match key_event.code {
            KeyCode::Esc | KeyCode::Char('h') => {
                let parent_dir = get_parent_dir(&self.current_dir)?
                    .into_os_string()
                    .into_string()
                    .unwrap();
                match parent_dir.cmp(&self.current_dir) {
                    std::cmp::Ordering::Equal => {}
                    _ => {
                        self.current_dir = parent_dir;
                        self.index = 0;
                        self.files = get_files(&self.current_dir)?;
                        self.update_sub_files();
                    }
                }
            }
            KeyCode::Enter | KeyCode::Char('l') => {
                if !self.files.is_empty() {
                    if (0..self.files.len()).contains(&self.index) {
                        let sub_file = &self.files[self.index];
                        match sub_file.file_type {
                            FileType::Dir => {
                                self.current_dir
                                    .push_str(&(String::from("/") + &sub_file.name));
                                self.update();
                            }
                            _ => {}
                        }
                    }
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if !self.files.is_empty() {
                    if self.index == 0 {
                        self.index = self.files.len() - 1;
                    } else {
                        self.index -= 1;
                    }
                } else {
                    self.index = 0;
                }

                self.update_sub_files();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.files.is_empty() {
                    if self.index == self.files.len() - 1 {
                        self.index = 0;
                    } else {
                        self.index += 1;
                    }
                } else {
                    self.index = 0;
                }

                self.update_sub_files();
            }
            KeyCode::Char('H') => {
                // FIX: so far I think filter_hidden_files might be useless
                // now i don't think it's useless xd
                self.filter_hidden_file = !self.filter_hidden_file;
                if self.filter_hidden_file {
                    self.files = filter_hidden_files(self.files.to_vec());
                } else {
                    self.files = get_files(&self.current_dir)?;
                }
                self.index = 0;
                self.update_sub_files();
            }
            KeyCode::Char(' ') => {
                // NOTE: if the pointted sub file is dir then it will be the finnal working dir
                if !self.files.is_empty() {
                    match self.files[self.index].file_type {
                        FileType::Dir => {
                            self.exit();
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.output = false;
                self.exit();
            }
            _ => {}
        }
        Ok(())
    }

    /// NOTE: update based on the self.current_dir
    fn update(&mut self) {
        self.files = self.sub_files.to_vec();
        self.index = 0;
        self.update_sub_files();
    }

    /// NOTE: update based on the self.index
    fn update_sub_files(&mut self) {
        self.sub_files = if !self.files.is_empty() {
            match self.files[self.index].file_type {
                FileType::Dir => {
                    get_files(&(self.current_dir.clone() + "/" + &self.files[self.index].name))
                        .unwrap()
                }
                FileType::File => Vec::new(),
            }
        } else {
            Vec::new()
        };
    }

    pub fn finnal_dir(&self) -> String {
        self.current_dir.clone() + "/" + &self.files[self.index].name
    }
}
