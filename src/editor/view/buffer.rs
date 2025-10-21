use super::line::Line;
use super::Location;
use std::fs::read_to_string;

#[derive(Default)]
pub struct Buffer {
  pub lines: Vec<Line>,
}

impl Buffer {
  pub fn load(filename: &str) -> Result<Self, std::io::Error> {
    Ok(Self {
      lines: read_to_string(filename)?.lines().map(Line::from).collect(),
    })
  }

  pub fn insert_char(&mut self, character: char, location: &Location) {
    if location.line_index > self.lines.len() {
      return;
    }

    if location.line_index == self.lines.len() {
      self.lines.push(Line::from(&character.to_string()));
    } else if let Some(line) = self.lines.get_mut(location.line_index) {
      line.insert_char(character, location.grapheme_index);
    }
  }
}
