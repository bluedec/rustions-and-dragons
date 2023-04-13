use std::io;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::sync::{ Arc, Mutex };

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::{ KeyCode::{ Esc }};
use crossterm::execute;
use crossterm::terminal::{ disable_raw_mode, enable_raw_mode };
use crossterm::cursor::{ Hide, Show };

use magic;


fn main() -> io::Result<()> {
    magic::clean();

    enable_raw_mode()?;
    // executes the loading bar. 
    //screens::intro(&mut stdout)?;
    magic::clean();

    screens::start_quit(); 
    screens::new_run();
    screens::choose_race();

    execute!(io::stdout(), crossterm::cursor::Show)?;
    magic::clean();

    let mut flag = true;
    let mut player_name = String::new();
    while flag == true {
        player_name = screens::ask_name()?;
        if let Ok(false) = screens::confirm_name(&player_name) {
            continue
        } else {
            flag = false
        }

    }
    println!("Name confirmed: {}\r", player_name);
    enable_raw_mode()?;

    magic::close()?;
    Ok(())
}


