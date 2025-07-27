use super::terminal::{Position, Size, Terminal};
mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
  buffer: Buffer,
  needs_redraw: bool,
  size: Size,
}

impl Default for View {
  fn default() -> Self {
    Self {
      buffer: Buffer::default(),
      needs_redraw: true,
      size: Terminal::size().unwrap_or_default(),
    }
  }
}

impl View {
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

  pub fn render(&mut self) -> Result<(), std::io::Error> {
    if !self.needs_redraw {
      return Ok(());
    }

    let Size { width, height } = self.size;
    if width == 0 || height == 0 {
      return Ok(());
    }

    #[allow(clippy::integer_division)]
    let vertical_center = height / 2;

    for current_row in 0..height {
      if let Some(line) = self.buffer.lines.get(current_row) {
        let truncated_line = if line.len() >= width {
          &line[0..width]
        } else {
          line
        };
        Self::render_line(current_row, truncated_line)?;
      } else if current_row == vertical_center && self.buffer.is_empty() {
        Self::render_line(current_row, &Self::get_welcome_message_string(width))?;
      } else {
        Self::render_line(current_row, "~")?;
      }
    }
    self.needs_redraw = false;

    Ok(())
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

  fn render_line(at: usize, line: &str) -> Result<(), std::io::Error> {
    Terminal::move_cursor_to(Position { x: 0, y: at })?;
    Terminal::clear_line()?;
    Terminal::print(line)?;
    Ok(())
  }
}
