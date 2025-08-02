mod buffer;
mod line;

use super::{
  editorcommand::{Direction, EditorCommand},
  terminal::{Position, Size, Terminal},
};
use buffer::Buffer;
use line::Line;
use std::cmp;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
pub struct Location {
  pub grapheme_index: usize,
  pub line_index: usize,
}

pub struct View {
  buffer: Buffer,
  location: Location,
  needs_redraw: bool,
  scroll_offset: Position,
  size: Size,
  virtual_grapheme_index: usize,
}

impl Default for View {
  fn default() -> Self {
    Self {
      buffer: Buffer::default(),
      location: Location::default(),
      needs_redraw: true,
      scroll_offset: Position::default(),
      size: Terminal::size().unwrap_or_default(),
      virtual_grapheme_index: 0,
    }
  }
}

impl View {
  pub fn render(&mut self) {
    if !self.needs_redraw {
      return;
    }

    let Size { width, height } = self.size;
    if width == 0 || height == 0 {
      return;
    }

    #[allow(clippy::integer_division)]
    let vertical_center = height / 2;

    for current_row in 0..height {
      let row_idx = current_row.saturating_add(self.scroll_offset.y);
      if let Some(line) = self.buffer.lines.get(row_idx) {
        let start = self.scroll_offset.x;
        let end = self.scroll_offset.x.saturating_add(width);
        Self::render_line(current_row, &line.get_visible_graphemes(start..end));
      } else if current_row == vertical_center && self.buffer.lines.is_empty() {
        Self::render_line(current_row, &Self::get_welcome_message_string(width));
      } else {
        Self::render_line(current_row, "~");
      }
    }
    self.needs_redraw = false;
  }

  pub fn handle_command(&mut self, command: EditorCommand) {
    match command {
      EditorCommand::Resize(size) => self.resize(size),
      EditorCommand::Move(direction) => self.move_text_location(&direction),
      EditorCommand::Quit => {}
    }
  }

  pub fn load(&mut self, filename: &str) {
    if let Ok(buffer) = Buffer::load(filename) {
      self.buffer = buffer;
      self.needs_redraw = true;
    }
  }

  pub fn cursor_position(&self) -> Position {
    self
      .text_location_to_position()
      .saturating_sub(self.scroll_offset)
  }

  fn move_text_location(&mut self, direction: &Direction) {
    let Size { height, .. } = self.size;

    match direction {
      Direction::Up => self.move_up(1),
      Direction::Down => self.move_down(1),
      Direction::Left => self.move_left(),
      Direction::Right => self.move_right(),
      Direction::PageUp => self.move_up(height.saturating_sub(1)),
      Direction::PageDown => self.move_down(height.saturating_sub(1)),
      Direction::Home => self.move_to_start_of_line(),
      Direction::End => self.move_to_end_of_line(),
    }

    self.scroll();
    self.needs_redraw = true;
  }

  fn move_up(&mut self, step: usize) {
    self.location.line_index = self.location.line_index.saturating_sub(step);
    self.snap_to_valid_grapheme();
  }

  fn move_down(&mut self, step: usize) {
    self.location.line_index = self.location.line_index.saturating_add(step);
    self.snap_to_valid_grapheme();
    self.snap_to_valid_line();
  }

  // clippy::arithmetic_side_effects: This function performs arithmetic calculations
  // after explicitly checking that the target value will be within bounds.
  #[allow(clippy::arithmetic_side_effects)]
  fn move_right(&mut self) {
    let line_width = self
      .buffer
      .lines
      .get(self.location.line_index)
      .map_or(0, Line::grapheme_count);
    if self.location.grapheme_index < line_width {
      self.location.grapheme_index += 1;
    } else {
      self.move_to_start_of_line();
      self.move_down(1);
    }
  }

  // clippy::arithmetic_side_effects: This function performs arithmetic calculations
  // after explicitly checking that the target value will be within bounds.
  #[allow(clippy::arithmetic_side_effects)]
  fn move_left(&mut self) {
    if self.location.grapheme_index > 0 {
      self.location.grapheme_index -= 1;
    } else {
      self.move_up(1);
      self.move_to_end_of_line();
    }
  }

  fn move_to_start_of_line(&mut self) {
    self.location.grapheme_index = 0;
    self.virtual_grapheme_index = 0;
  }

  fn move_to_end_of_line(&mut self) {
    self.location.grapheme_index = self
      .buffer
      .lines
      .get(self.location.line_index)
      .map_or(0, Line::grapheme_count);
    self.virtual_grapheme_index = self.location.grapheme_index;
  }

  // Ensures self.location.grapheme_index points to a valid grapheme index by snapping it to the left most grapheme if appropriate.
  // Doesn't trigger scrolling.
  fn snap_to_valid_grapheme(&mut self) {
    self.location.grapheme_index = self
      .buffer
      .lines
      .get(self.location.line_index)
      .map_or(0, |line| {
        cmp::min(line.grapheme_count(), self.location.grapheme_index)
      });
    self.virtual_grapheme_index = self.location.grapheme_index;
  }

  // Ensures self.location.line_index points to a valid line index by snapping it to the bottom most line if appropriate.
  // Doesn't trigger scrolling.
  fn snap_to_valid_line(&mut self) {
    self.location.line_index = cmp::min(self.location.line_index, self.buffer.lines.len());
  }

  fn text_location_to_position(&self) -> Position {
    let row = self.location.line_index;
    let col = self
      .buffer
      .lines
      .get(row)
      .map_or(0, |line| line.width_until(self.location.grapheme_index));
    Position { x: col, y: row }
  }

  fn scroll(&mut self) {
    let Size { width, height } = self.size;
    let Location {
      grapheme_index: col,
      line_index: row,
    } = self.location;
    let mut scroll_offset = self.scroll_offset;
    let mut offset_changed = false;

    if row < scroll_offset.y {
      scroll_offset.y = row;
      offset_changed = true;
    } else if row >= scroll_offset.y + height {
      scroll_offset.y = row - height + 1;
      offset_changed = true;
    }

    let actual_col = self
      .buffer
      .lines
      .get(row)
      .map_or(0, |line| line.width_until(col));

    if actual_col < scroll_offset.x {
      scroll_offset.x = actual_col;
      offset_changed = true;
    } else if actual_col >= scroll_offset.x + width {
      scroll_offset.x = actual_col - width + 1;
      offset_changed = true;
    }

    self.scroll_offset = scroll_offset;
    self.needs_redraw = offset_changed;
  }

  fn resize(&mut self, to: Size) {
    self.size = to;
    self.scroll();
    self.needs_redraw = true;
  }

  fn get_welcome_message_string(width: usize) -> String {
    let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
    let len = welcome_message.len();

    #[allow(clippy::integer_division)]
    let padding = (width.saturating_sub(len)) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));

    welcome_message = format!("~{spaces}{welcome_message}");
    welcome_message.truncate(width);

    welcome_message
  }

  fn render_line(at_y: usize, line_text: &str) {
    let result = Terminal::print_row(at_y, line_text);
    debug_assert!(result.is_ok(), "Failed to render line");
  }
}
