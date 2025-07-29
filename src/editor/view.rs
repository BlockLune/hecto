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
  needs_redraw: bool,
  size: Size,
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
    }
  }
}

impl View {
  pub fn move_cursor(&mut self, key_code: KeyCode) {
    let Size { width: _, height } = self.size;
    let Location {
      mut column,
      mut row,
    } = self.location;

    match key_code {
      KeyCode::Up => {
        row = row.saturating_sub(1);
      }
      KeyCode::Down => {
        if row < self.buffer.lines.len() {
          row = row.saturating_add(1);
        }
      }
      KeyCode::Left => {
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
      }
      KeyCode::Right => {
        if let Some(line) = self.buffer.lines.get(row) {
          if column < line.len() {
            column += 1;
          } else if row < self.buffer.lines.len() {
            row += 1;
            column = 0;
          }
        }
      }
      KeyCode::PageUp => {
        row = row.saturating_sub(height);
      }
      KeyCode::PageDown => {
        row = row.saturating_add(height);
        if row > self.buffer.lines.len() {
          row = self.buffer.lines.len();
        }
      }
      KeyCode::Home => {
        column = 0;
      }
      KeyCode::End => {
        if let Some(line) = self.buffer.lines.get(row) {
          column = line.len();
        }
      }
      _ => {}
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

    if row < scroll_offset.row {
      scroll_offset.row = row;
    } else if row >= scroll_offset.row + height {
      scroll_offset.row = row - height + 1;
    }

    if column < scroll_offset.column {
      scroll_offset.column = column;
    } else if column >= scroll_offset.column + width {
      scroll_offset.column = column - width + 1;
    }
    self.scroll_offset = scroll_offset;
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

    for current_row in 0..height {
      let row_idx = self.scroll_offset.row + current_row;
      if let Some(line) = self.buffer.lines.get(row_idx) {
        let start = self.scroll_offset.column;
        let end = self.scroll_offset.column.saturating_add(width);
        let truncated_line = if start > line.len() {
          ""
        } else {
          &line[start..std::cmp::min(end, line.len())]
        };
        Self::render_line(current_row, truncated_line);
      } else if current_row == vertical_center && self.buffer.lines.is_empty() {
        Self::render_line(current_row, &Self::get_welcome_message_string(width));
      } else {
        Self::render_line(current_row, "~");
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
