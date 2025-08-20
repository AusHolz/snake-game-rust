#![allow(unused)]

use std::io::{self, Write};
use std::time::Duration;

use crossterm::{
    execute, 
    style::{self, Stylize, Color, Print}, 
    terminal, 
    cursor,
    event::{self, read, poll, KeyCode, Event}
};

use snake_game;

fn main() -> io::Result<()> {

    let (start_columns, start_rows) = terminal::size()?;
    let game_rows = 5;
    let game_columns = 5;
    let mut output_buffer = io::stdout();

    // Set up the alternate terminal screen
    execute!(
        output_buffer,
        terminal::EnterAlternateScreen,
        cursor::Hide
    )?;

    // Flush the event buffer for clean input handling
    while event::poll(Duration::from_millis(0))? {
        let _ = event::read()?;
    }

    // Main game loop
    loop {
        if poll(Duration::from_millis(400))? {
            let ev = read()?;
            if let Event::Key(k) = ev {
                match k.code {
                    _ => (),
                }
            }
        } else {

        }
    }

    // Clean up and finish
    execute!(
        output_buffer,
        terminal::LeaveAlternateScreen,
    )?;

    Ok(())
}