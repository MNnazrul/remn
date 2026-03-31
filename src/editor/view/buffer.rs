use std::fs::{read_to_string, File};
use std::io::Error;
use std::io::Write;

use crate::editor::fileinfo::FileInfo;

use super::line::Line;
use super::Location;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub file_info: FileInfo,
    pub dirty: bool,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self {
            lines,
            file_info: FileInfo::from(file_name),
            dirty: false,
        })
    }
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(path) = &self.file_info.path {
            let mut file = File::create(path)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
            self.dirty = false;
        }
        Ok(())
    }
    pub fn save_as(&mut self, file_name: &str) {
        self.file_info = FileInfo::from(file_name);
        let _ = self.save();
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn height(&self) -> usize {
        self.lines.len()
    }
    pub fn insert_char(&mut self, character: char, at: Location) {
        if at.line_index > self.height() {
            return;
        }
        if at.line_index == self.height() {
            self.lines.push(Line::from(&character.to_string()));
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index);
            self.dirty = true;
        }
    }
    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get(at.line_index) {
            if at.grapheme_index >= line.grapheme_count()
                && self.height() > at.line_index.saturating_add(1)
            {
                let next_line = self.lines.remove(at.line_index.saturating_add(1));
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_index].append(&next_line);
                self.dirty = true;
            } else if at.grapheme_index < line.grapheme_count() {
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_index].delete(at.grapheme_index);
                self.dirty = true;
            }
        }
    }
    pub fn search_forward(&self, query: &str, from: Location) -> Option<Location> {
        if query.is_empty() {
            return None;
        }
        // Search from current position on current line
        if let Some(line) = self.lines.get(from.line_index) {
            if let Some(gi) = line.search_forward(query, from.grapheme_index) {
                return Some(Location {
                    line_index: from.line_index,
                    grapheme_index: gi,
                });
            }
        }
        // Search subsequent lines
        for li in (from.line_index + 1)..self.height() {
            if let Some(line) = self.lines.get(li) {
                if let Some(gi) = line.search_forward(query, 0) {
                    return Some(Location {
                        line_index: li,
                        grapheme_index: gi,
                    });
                }
            }
        }
        // Wrap around from the top
        for li in 0..=from.line_index {
            let start_gi = if li == from.line_index {
                from.grapheme_index
            } else {
                0
            };
            if let Some(line) = self.lines.get(li) {
                if let Some(gi) = line.search_forward(query, start_gi) {
                    return Some(Location {
                        line_index: li,
                        grapheme_index: gi,
                    });
                }
            }
        }
        None
    }

    pub fn insert_newline(&mut self, at: Location) {
        if at.line_index == self.height() {
            self.lines.push(Line::default());
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            let new = line.split(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new);
            self.dirty = true;
        }
    }
}
