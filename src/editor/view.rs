mod buffer;
mod line;
mod location;

use super::{
  editorcommand::{Direction, EditorCommand},
  terminal::{Position, Size, Terminal},
};
use buffer::Buffer;
use location::Location;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
  buffer: Buffer,
  needs_redraw: bool,
  size: Size,

  /// The column the user wants the cursor to be on.
  /// This is used to preserve the horizontal cursor position when moving vertically
  /// across lines of different lengths.
  virtual_column: usize,

  pub location: Location,
  pub scroll_offset: Location,
}

impl Default for View {
  fn default() -> Self {
    Self {
      buffer: Buffer::default(),
      location: Location::default(),
      needs_redraw: true,
      scroll_offset: Location::default(),
      size: Terminal::size().unwrap_or_default(),
      virtual_column: 0,
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
      let row_idx = current_row.saturating_add(self.scroll_offset.row);
      if let Some(line) = self.buffer.lines.get(row_idx) {
        let start = self.scroll_offset.column;
        let end = self.scroll_offset.column.saturating_add(width);
        Self::render_line(current_row, &line.get(start..end));
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

  pub fn get_position(&self) -> Position {
    self.location.subtract(&self.scroll_offset).into()
  }

  fn move_text_location(&mut self, direction: &Direction) {
    let Size { height, .. } = self.size;
    let Location {
      mut column,
      mut row,
    } = self.location;

    match direction {
      Direction::Up => {
        row = row.saturating_sub(1);
        column = self.virtual_column;
      }
      Direction::Down => {
        if row < self.buffer.lines.len() {
          row = row.saturating_add(1);
          column = self.virtual_column;
        }
      }
      Direction::Left => {
        if column > 0 {
          column -= 1;
        } else if row > 0 {
          row -= 1;
          if let Some(line) = self.buffer.lines.get(row) {
            column = line.len();
          } else {
            column = 0;
          }
        }
        self.virtual_column = column;
      }
      Direction::Right => {
        if let Some(line) = self.buffer.lines.get(row) {
          if column < line.len() {
            column += 1;
          } else if row < self.buffer.lines.len() {
            row += 1;
            column = 0;
          }
        }
        self.virtual_column = column;
      }
      Direction::PageUp => {
        row = row.saturating_sub(height);
      }
      Direction::PageDown => {
        row = row.saturating_add(height);
        if row > self.buffer.lines.len() {
          row = self.buffer.lines.len();
        }
      }
      Direction::Home => {
        column = 0;
        self.virtual_column = column;
      }
      Direction::End => {
        if let Some(line) = self.buffer.lines.get(row) {
          column = line.len();
        }
        self.virtual_column = column;
      }
    }

    if let Some(line) = self.buffer.lines.get(row) {
      if column > line.len() {
        column = line.len();
      }
    } else {
      column = 0;
    }

    self.location = Location { column, row };
    self.scroll();
    self.needs_redraw = true;
  }

  fn scroll(&mut self) {
    let Size { width, height } = self.size;
    let Location { column, row } = self.location;
    let mut scroll_offset = self.scroll_offset;
    let mut offset_changed = false;

    if row < scroll_offset.row {
      scroll_offset.row = row;
      offset_changed = true;
    } else if row >= scroll_offset.row + height {
      scroll_offset.row = row - height + 1;
      offset_changed = true;
    }

    if column < scroll_offset.column {
      scroll_offset.column = column;
      offset_changed = true;
    } else if column >= scroll_offset.column + width {
      scroll_offset.column = column - width + 1;
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
