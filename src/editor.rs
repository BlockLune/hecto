use crossterm::event::read;
use crossterm::event::Event::Key;
use crossterm::event::KeyCode::Char;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }
    pub fn run(&self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }
    fn repl(&self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(event) = read()? {
                println!("{event:?} \r");
                if let Char(c) = event.code {
                    if c == 'q' {
                        break;
                    }
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
