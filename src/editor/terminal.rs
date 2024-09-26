use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Write};

pub struct Terminal {}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }
    pub fn print(string: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size { height, width })
    }
    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }
}
