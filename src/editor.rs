use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::cmp::min;
mod terminal;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
struct Location {
    pub row: usize,
    pub column: usize,
}

pub struct View {}

impl View {
    fn draw_hello_world() -> Result<(), std::io::Error> {
        Terminal::print("Hello, World!")?;
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

    pub fn render() -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else if current_row == 0 {
                Self::draw_hello_world()?;
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

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    cursor_location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn move_cursor(&mut self, key_code: KeyCode) -> Result<(), std::io::Error> {
        let Location {
            mut row,
            mut column,
        } = self.cursor_location;
        let Size { height, width } = Terminal::size()?;
        match key_code {
            // up: cursor move up
            KeyCode::Up => {
                column = column.saturating_sub(1);
            }
            // down: cursor move down
            KeyCode::Down => {
                column = min(height.saturating_sub(1), column.saturating_add(1));
            }
            // left: cursor move left
            KeyCode::Left => {
                row = row.saturating_sub(1);
            }
            // right: cursor move right
            KeyCode::Right => {
                row = min(width.saturating_sub(1), row.saturating_add(1));
            }
            // pageup: cursor move to top
            KeyCode::PageUp => {
                row = 0;
            }
            // pagedown: cursor move to bottom
            KeyCode::PageDown => {
                row = height.saturating_sub(1);
            }
            // home: cursor move to leftmost
            KeyCode::Home => {
                column = 0;
            }
            // end: cursor move to rightmost
            KeyCode::End => {
                column = width.saturating_sub(1);
            }
            _ => {}
        }
        self.cursor_location = Location { row, column };
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                // ctrl + q: quit
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                // up, down, left, right, pageup, pagedown, home, end: move cursor
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End => {
                    self.move_cursor(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            View::render()?;
            Terminal::move_cursor_to(Position {
                x: self.cursor_location.row,
                y: self.cursor_location.column,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }


}
