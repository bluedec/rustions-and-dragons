use std::io;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::sync::{ Arc, Mutex };

use screens::canvas;

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::{ KeyCode::{ Esc }};
use crossterm::execute;
use crossterm::terminal::{ disable_raw_mode, enable_raw_mode };
use crossterm::terminal::{ Clear, ClearType };
use crossterm::cursor::{ self, Hide, Show };

use magic;


fn main() -> io::Result<()> {
    magic::clean();
    screens::intro();

    enable_raw_mode()?;
    std::thread::spawn(|| {
        let mut counter = 0;
        let mut quit = false;
        while !quit {
            let (width, height) = crossterm::terminal::size().unwrap();
            magic::wait_a_sec(1);

            counter += 1;
            execute!(
                io::stdout(),
                cursor::SavePosition,
                cursor::MoveTo(width - 20, height - 20),
            );
            println!("{}\r", counter);
            execute!(io::stdout(), cursor::RestorePosition);
        }
    });
    //screens::intro()?;
    magic::clean();

    screens::start_quit(); 
    screens::new_run();
    screens::choose_race();

    execute!(io::stdout(), crossterm::cursor::Show)?;
    magic::clean();

    let mut flag = true;
    let mut player_name = String::new();
    while flag == true {
        println!("{player_name}");
        player_name = screens::ask_name()?;
        if let Ok(false) = screens::confirm_name(&player_name) {
            continue
        } else {
            flag = false
        }

    }
    println!("Name confirmed: {}\r", player_name);
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    //screens::intro()?;
    magic::close()?;
    Ok(())
}


