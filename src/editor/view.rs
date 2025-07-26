use super::terminal::{Size, Terminal};
mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
  buffer: Buffer,
}

// draw utilitiy functions
impl View {
  fn draw_content(content: &str) -> Result<(), std::io::Error> {
    Terminal::print(content)?;
    Ok(())
  }

  fn draw_welcome_message() -> Result<(), std::io::Error> {
    let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
    let width = Terminal::size()?.width;
    let len = welcome_message.len();

    #[allow(clippy::integer_division)]
    let padding = (width.saturating_sub(len)) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));

    welcome_message = format!("~{spaces}{welcome_message}");
    welcome_message.truncate(width);
    Terminal::print(&welcome_message)?;
    Ok(())
  }

  fn draw_empty_row() -> Result<(), std::io::Error> {
    Terminal::print("~")?;
    Ok(())
  }
}

// buffer loading
impl View {
  pub fn load(&mut self, filename: &str) {
    if let Ok(buffer) = Buffer::load(filename) {
      self.buffer = buffer;
    }
  }
}

// render
impl View {
  pub fn render(&self) -> Result<(), std::io::Error> {
    let Size { height, .. } = Terminal::size()?;
    for current_row in 0..height {
      Terminal::clear_line()?;

      #[allow(clippy::integer_division)]
      if self.buffer.is_empty() && current_row == height / 3 {
        Self::draw_welcome_message()?;
      } else if let Some(line) = self.buffer.lines.get(current_row) {
        Self::draw_content(line)?;
      } else {
        Self::draw_empty_row()?;
      }

      if current_row.saturating_add(1) < height {
        Terminal::print("\r\n")?;
      }
    }
    Ok(())
  }
}
