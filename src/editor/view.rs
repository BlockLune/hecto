use super::terminal::{Position, Size, Terminal};
mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
  buffer: Buffer,
  current_width: usize,
  current_height: usize,
}

impl View {
  pub fn load(&mut self, filename: &str) {
    if let Ok(buffer) = Buffer::load(filename) {
      self.buffer = buffer;
    }
  }

  pub fn render(&mut self) -> Result<(), std::io::Error> {
      let Size { width, height } = Terminal::size()?;

      if width == 0 || height == 0 {
          return Ok(());
      }

      if width == self.current_width && height == self.current_height {
          return Ok(());
      }

      self.current_width = width;
      self.current_height = height;
      self.render_screen(width, height)?;

      Ok(())
  }

  fn get_welcome_message_string(width: usize) -> Result<String, std::io::Error> {
    let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
    let len = welcome_message.len();

    #[allow(clippy::integer_division)]
    let padding = (width.saturating_sub(len)) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));

    welcome_message = format!("~{spaces}{welcome_message}");
    welcome_message.truncate(width);

    Ok(welcome_message)
  }

  fn render_line(line: &str, width: usize) -> Result<(), std::io::Error> {
    let safe_line = if line.len() > width {
      &line[0..width]
    } else {
      line
    };

    Terminal::print(safe_line)?;
    Ok(())
  }

  fn render_screen(&self, width: usize, height: usize) -> Result<(), std::io::Error> {
    for i in 0..height {
      Terminal::clear_line()?;

      #[allow(clippy::integer_division)]
      if self.buffer.is_empty() && i == height / 3 {
        Self::render_line(&(Self::get_welcome_message_string(width)?), width)?;
      } else if let Some(line) = self.buffer.lines.get(i) {
        Self::render_line(line, width)?;
      } else {
        Self::render_line("~", width)?;
      }
      if i.saturating_add(1) < height {
        Terminal::move_cursor_to(Position { x: 0, y: i + 1 })?;
      }
    }

    Ok(())
  }
}
