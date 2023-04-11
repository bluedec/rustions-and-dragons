use std::io;

use crossterm::terminal::enable_raw_mode;

use magic::{ clean, close };


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    clean();

    screens::start_quit(); 

    close()?;
    Ok(())
}


