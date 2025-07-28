use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{
  disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
  LeaveAlternateScreen,
};
use crossterm::{queue, Command};
use std::io::{stdout, Write};

pub struct Terminal;

#[derive(Copy, Clone, Default)]
pub struct Position {
  pub x: usize,
  pub y: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Size {
  pub width: usize,
  pub height: usize,
}

impl Terminal {
  pub fn enter_alternate_screen() -> Result<(), std::io::Error> {
    Self::queue_command(EnterAlternateScreen)?;
    Ok(())
  }

  pub fn leave_alternate_screen() -> Result<(), std::io::Error> {
    Self::queue_command(LeaveAlternateScreen)?;
    Ok(())
  }

  pub fn terminate() -> Result<(), std::io::Error> {
    Self::leave_alternate_screen()?;
    Self::show_cursor()?;
    Self::execute()?;
    disable_raw_mode()?;
    Ok(())
  }

  pub fn initialize() -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    Self::enter_alternate_screen()?;
    Self::clear_screen()?;
    Self::execute()?;
    Ok(())
  }

  pub fn clear_screen() -> Result<(), std::io::Error> {
    Self::queue_command(Clear(ClearType::All))?;
    Ok(())
  }

  pub fn clear_line() -> Result<(), std::io::Error> {
    Self::queue_command(Clear(ClearType::CurrentLine))?;
    Ok(())
  }

  pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
    #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
    Self::queue_command(MoveTo(position.x as u16, position.y as u16))?;
    Ok(())
  }

  pub fn hide_cursor() -> Result<(), std::io::Error> {
    Self::queue_command(Hide)?;
    Ok(())
  }

  pub fn show_cursor() -> Result<(), std::io::Error> {
    Self::queue_command(Show)?;
    Ok(())
  }

  pub fn print(string: &str) -> Result<(), std::io::Error> {
    Self::queue_command(Print(string))?;
    Ok(())
  }

  pub fn print_row(row: usize, line_text: &str) -> Result<(), std::io::Error> {
    Self::move_cursor_to(Position { x: 0, y: row })?;
    Self::clear_line()?;
    Self::print(line_text)?;
    Ok(())
  }

  pub fn size() -> Result<Size, std::io::Error> {
    let (width_u16, height_u16) = size()?;
    #[allow(clippy::as_conversions)]
    let height = height_u16 as usize;
    #[allow(clippy::as_conversions)]
    let width = width_u16 as usize;
    Ok(Size { width, height })
  }

  pub fn execute() -> Result<(), std::io::Error> {
    stdout().flush()?;
    Ok(())
  }

  fn queue_command<T: Command>(command: T) -> Result<(), std::io::Error> {
    queue!(stdout(), command)?;
    Ok(())
  }
}
