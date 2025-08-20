use std::io::{self, Write};
use crossterm::{execute, terminal::{ScrollUp, SetSize, size}};
use std::time::Duration;

fn main() -> io::Result<()> {
    let (cols, rows) = size()?;
    // Resize terminal and scroll up.
    execute!(
        io::stdout(),
        SetSize(10, 10),
        ScrollUp(5)
    )?;

    std::thread::sleep(Duration::new(2, 0));

    // Be a good citizen, cleanup
    execute!(io::stdout(), SetSize(cols, rows))?;
    Ok(())
}