use crate::editor::view::Location;
use crate::editor::view::buffer::Buffer;
use std::sync::{Arc, Mutex};

/// Trait for editor commands that can be executed and undone
pub trait Command: Send + Sync {
    fn execute(&self, buffer: &mut Buffer);
    fn undo(&self, buffer: &mut Buffer);
}

/// Command for inserting a character
pub struct InsertCharCommand {
    pub character: char,
    pub location: Location,
}

impl Command for InsertCharCommand {
    fn execute(&self, buffer: &mut Buffer) {
        buffer.insert_char(self.character, self.location);
    }

    fn undo(&self, buffer: &mut Buffer) {
        // To undo an insert, we delete the character at the same location
        buffer.delete(self.location);
    }
}

/// Command for deleting a character
pub struct DeleteCharCommand {
    pub location: Location,
    pub deleted_character: char,
}

impl Command for DeleteCharCommand {
    fn execute(&self, buffer: &mut Buffer) {
        buffer.delete(self.location);
    }

    fn undo(&self, buffer: &mut Buffer) {
        // To undo a delete, we insert the character back at the same location
        buffer.insert_char(self.deleted_character, self.location);
    }
}

/// Command for inserting a newline
pub struct InsertNewlineCommand {
    pub location: Location,
}

impl Command for InsertNewlineCommand {
    fn execute(&self, buffer: &mut Buffer) {
        buffer.insert_newline(self.location);
    }

    fn undo(&self, buffer: &mut Buffer) {
        // To undo a newline, we delete the newline and merge the lines
        // We need to delete at the position where the newline was inserted
        let delete_location = Location {
            line_index: self.location.line_index,
            grapheme_index: self.location.grapheme_index,
        };
        buffer.delete(delete_location);
    }
}

/// Command for deleting a newline (merging lines)
pub struct DeleteNewlineCommand {
    pub location: Location,
    pub merged_content: String,
}

impl Command for DeleteNewlineCommand {
    fn execute(&self, buffer: &mut Buffer) {
        buffer.delete(self.location);
    }

    fn undo(&self, buffer: &mut Buffer) {
        // To undo a delete newline, we need to reinsert the newline
        buffer.insert_newline(self.location);
    }
}

/// A generic command that can hold any specific command type
pub enum EditorCommandEnum {
    InsertChar(InsertCharCommand),
    DeleteChar(DeleteCharCommand),
    InsertNewline(InsertNewlineCommand),
    DeleteNewline(DeleteNewlineCommand),
}

impl EditorCommandEnum {
    pub fn execute(&self, buffer: &mut Buffer) {
        match self {
            EditorCommandEnum::InsertChar(cmd) => cmd.execute(buffer),
            EditorCommandEnum::DeleteChar(cmd) => cmd.execute(buffer),
            EditorCommandEnum::InsertNewline(cmd) => cmd.execute(buffer),
            EditorCommandEnum::DeleteNewline(cmd) => cmd.execute(buffer),
        }
    }

    pub fn undo(&self, buffer: &mut Buffer) {
        match self {
            EditorCommandEnum::InsertChar(cmd) => cmd.undo(buffer),
            EditorCommandEnum::DeleteChar(cmd) => cmd.undo(buffer),
            EditorCommandEnum::InsertNewline(cmd) => cmd.undo(buffer),
            EditorCommandEnum::DeleteNewline(cmd) => cmd.undo(buffer),
        }
    }
}

/// Wrapper for Arc<Mutex<EditorCommandEnum>> to make it easier to store in collections
#[derive(Clone)]
pub struct CommandWrapper(pub Arc<Mutex<EditorCommandEnum>>);

impl CommandWrapper {
    pub fn new(command: EditorCommandEnum) -> Self {
        CommandWrapper(Arc::new(Mutex::new(command)))
    }

    pub fn execute(&self, buffer: &mut Buffer) {
        let mut command = self.0.lock().unwrap();
        command.execute(buffer);
    }

    pub fn undo(&self, buffer: &mut Buffer) {
        let mut command = self.0.lock().unwrap();
        command.undo(buffer);
    }
}