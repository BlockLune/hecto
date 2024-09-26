use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;

use std::io;
use std::io::Read;

fn main() {
    // enable raw mode of the terminal
    // i.e. disable the cooked mode of the terminal
    enable_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}
