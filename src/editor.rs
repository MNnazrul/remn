use crossterm::event::{
    read,
    Event::{self},
    KeyCode, KeyEvent, KeyEventKind,
};
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
    thread::current,
};

mod documentstatus;
mod editorcommand;
mod fileinfo;
mod messagebar;
mod statusbar;
mod terminal;
use documentstatus::DocumentStatus;
use uicomponent::UIComponent;
mod uicomponent;
use statusbar::StatusBar;
mod view;
use terminal::Terminal;
use view::View;

use self::{messagebar::MessageBar, terminal::Size};
use crate::editor::editorcommand::EditorCommand;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default, PartialEq)]
enum PromptMode {
    #[default]
    None,
    SaveAs(String),
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    terminal_size: Size,
    title: String,
    prompt_mode: PromptMode,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;

        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.resize(size);

        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            editor.view.load(file_name);
        }

        Ok(editor)
    }

    fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });
        self.message_bar.resize(Size {
            height: 1,
            width: size.width,
        });
        self.status_bar.resize(Size {
            height: 1,
            width: size.width,
        });
    }

    fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);

        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
            let status = self.view.get_status();
            self.status_bar.update_status(status);
        }
    }

    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if !should_process {
            #[cfg(debug_assertions)]
            {
                panic!("Received and discarded unsupported or non-press event.")
            }
            #[cfg(not(debug_assertions))]
            return;
        }

        // Handle prompt mode input
        if self.prompt_mode != PromptMode::None {
            self.handle_prompt_event(event);
            return;
        }

        if let Ok(command) = EditorCommand::try_from(event) {
            match command {
                EditorCommand::Quit => self.should_quit = true,
                EditorCommand::Resize(size) => self.resize(size),
                EditorCommand::SaveAs => {
                    self.prompt_mode = PromptMode::SaveAs(String::new());
                    self.message_bar
                        .update_message("Save as: ".to_string());
                }
                _ => self.view.handle_command(command),
            }
        }
    }

    fn handle_prompt_event(&mut self, event: Event) {
        if let Event::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Esc => {
                    self.prompt_mode = PromptMode::None;
                    self.message_bar.update_message(String::new());
                }
                KeyCode::Enter => {
                    if let PromptMode::SaveAs(ref filename) = self.prompt_mode {
                        if !filename.is_empty() {
                            let filename = filename.clone();
                            self.view.save_as(&filename);
                            self.message_bar
                                .update_message(format!("Saved as: {filename}"));
                        }
                    }
                    self.prompt_mode = PromptMode::None;
                }
                KeyCode::Backspace => {
                    if let PromptMode::SaveAs(ref mut input) = self.prompt_mode {
                        input.pop();
                        self.message_bar
                            .update_message(format!("Save as: {input}"));
                    }
                }
                KeyCode::Char(c) => {
                    if let PromptMode::SaveAs(ref mut input) = self.prompt_mode {
                        input.push(c);
                        self.message_bar
                            .update_message(format!("Save as: {input}"));
                    }
                }
                _ => {}
            }
        }
    }

    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }
        let _ = Terminal::hide_caret();
        self.message_bar
            .render(self.terminal_size.height.saturating_sub(1));
        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }
        if self.terminal_size.height > 2 {
            self.view.render(0);
        }
        let _ = Terminal::move_caret_to(self.view.caret_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
