mod buffer;

use super::terminal::{Size, Terminal};
use buffer::Buffer;
use crossterm::event::KeyCode;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
pub struct Location {
  pub column: usize,
  pub row: usize,
}

pub struct View {
  buffer: Buffer,
  location: Location,
  needs_redraw: bool,
  scroll_offset: Location,
  size: Size,
}

impl Default for View {
  fn default() -> Self {
    Self {
      buffer: Buffer::default(),
      location: Location::default(),
      needs_redraw: true,
      scroll_offset: Location::default(),
      size: Terminal::size().unwrap_or_default(),
    }
  }
}

impl View {
  pub fn move_cursor(&mut self, key_code: KeyCode) {
    let Size { width, height } = self.size;
    let Location {
      mut column,
      mut row,
    } = self.location;

    let row_range = (self.scroll_offset.row, self.scroll_offset.row + height);
    let column_range = (self.scroll_offset.column, self.scroll_offset.column + width);

    match key_code {
      KeyCode::Up => {
        row = row.saturating_sub(1);
        if row < row_range.0 {
          self.scroll_offset.row -= 1;
        }
      }
      KeyCode::Down => {
        row = row.saturating_add(1);
        if row >= row_range.1 {
          self.scroll_offset.row += 1;
        }
      }
      KeyCode::Left => {
        column = column.saturating_sub(1);
        if column < column_range.0 {
          self.scroll_offset.column -= 1;
        }
      }
      KeyCode::Right => {
        column = column.saturating_add(1);
        if column > column_range.1 {
          self.scroll_offset.column += 1;
        }
      }
      KeyCode::PageUp => {
        row = row_range.0;
      }
      KeyCode::PageDown => {
        row = row_range.1.saturating_sub(1);
      }
      KeyCode::Home => {
        column = column_range.0;
      }
      KeyCode::End => {
        column = column_range.1.saturating_sub(1);
      }
      _ => {}
    }

    self.location = Location { column, row };
    self.needs_redraw = true;
  }

  pub fn resize(&mut self, to: Size) {
    self.size = to;
    self.needs_redraw = true;
  }

  pub fn load(&mut self, filename: &str) {
    if let Ok(buffer) = Buffer::load(filename) {
      self.buffer = buffer;
      self.needs_redraw = true;
    }
  }

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

    for current_row in self.scroll_offset.row..(self.scroll_offset.row + height) {
      let current_y = current_row - self.scroll_offset.row;
      if let Some(line) = self.buffer.lines.get(current_row) {
        let truncated_line = if line.len() >= width {
          &line[self.scroll_offset.column..(self.scroll_offset.column + width)]
        } else {
          line
        };
        Self::render_line(current_y, truncated_line);
      } else if current_y == vertical_center && self.buffer.is_empty() {
        Self::render_line(current_y, &Self::get_welcome_message_string(width));
      } else {
        Self::render_line(current_y, "~");
      }
    }
    self.needs_redraw = false;
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
