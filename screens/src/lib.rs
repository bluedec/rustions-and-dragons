
use std::io;
use std::io::Write;
use std::io::Stdout;
use std::sync::mpsc;
use std::sync::{ Arc, Mutex };

use crossterm::cursor::SavePosition;
use crossterm::terminal;
use crossterm::event::Event;
use crossterm::{ event, cursor, execute };
use crossterm::terminal::{ Clear, ClearType };
use crossterm::terminal::{ enable_raw_mode, disable_raw_mode};
use crossterm::terminal::{ size };
use crossterm::cursor::{ Show, MoveRight };

use magic::*;

pub fn intro(stdout: &mut Stdout) -> io::Result<()> {
    execute!(stdout, cursor::Hide).unwrap();
    execute!(stdout, cursor::Show)?;
    clean();
    println!("\r");
    let load = "[>------------------------------------<]".to_string();
    for char in load.chars() {
        print!("{}", char);
        stdout.flush()?;
        magic::wait_a_milli(20);
    }
    let mut loaded = String::with_capacity(load.len()); 
    for c in load.chars() {
        loaded.push(if c == '-' { '#' } else { c });
        print!("\r{}", loaded);
        stdout.flush()?;
        magic::wait_a_milli(50);
    }
    magic::wait_a_milli(10);
    execute!(stdout, cursor::MoveTo(16, 1));
    println!(" Click!\r");
    magic::wait_a_milli(170);
    Ok(())

}

pub fn boilerplate() {
    // each item represents a basic necessity for most menus, it's in order.
    // do options for menu
    // default_position (index) of the default option as a u16
    // max value of the current_position (options.len() - 1)
    // coordinates if using show_options_at()
    // loop {
    // magic::show_options(&options, default_position)
    // magic::show_options_at(&options, default_position, coordinates)
    // handle events
    // }
    //
}

pub fn start_quit() {
    let mut stdout = io::stdout();
    let mut current_option = 0;
    let options = vec![") Start", ") Quit"];
    let options_len: u16 = options.len() as u16;
    loop {
        clean();
        println!("Rustions & Dragons\r");
        println!("-------------------------------\r");

        magic::show_options(&options, current_option);

        let input = event::read().unwrap();

        match read_up_down(&input, current_option, options_len) {
            UpDown::Down => {
                current_option += 1;
                continue
            },
            UpDown::Up => {
                current_option -= 1;
                continue
            },
            UpDown::Nil => {},
        };

        if input == Event::Key(event::KeyCode::Enter.into()) {
            execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
            if current_option == 0 {
                break
            }
            if current_option == 1 {
                close().unwrap();
            }
        }
    }
}


pub fn new_run() {
    let options = vec![") New Run", ") Return"];
    let mut current_option = 0;
    let max: u16 = options.len() as u16;
    loop {
        clean();
        magic::simple_line_with_title("");
        magic::show_options(&options, current_option);

        let input = event::read().unwrap();
        match read_up_down(&input, current_option, max) {
            UpDown::Down => {
                current_option += 1;
                continue
            },
            UpDown::Up => {
                current_option -= 1;
                continue
            },
            UpDown::Nil => {},
        };
        if input == Event::Key(event::KeyCode::Enter.into()) {
            if current_option == 0 {
                break
            }
            if current_option == 1 {
                start_quit();
            }

        }
    }

}
pub fn choose_race() {
    let options = vec![") Human", ") Blocked", ") Blocked", "\n) Return"];
    let mut current_option = 0;
    let options_len: u16 = options.len() as u16;
    loop {
        clean();
        magic::simple_line_with_title(&"What are you?".to_string()); 
        magic::show_options(&options, current_option);

        let input = event::read().unwrap();

        match read_up_down(&input, current_option, options_len) {
            UpDown::Down => {
                current_option += 1;
                continue
            },
            UpDown::Up => {
                current_option -= 1;
                continue
            },
            UpDown::Nil => {},
        };


        if input == Event::Key(event::KeyCode::Enter.into()) {
            if current_option == 0 {
                break
            }
            if current_option == 1 {
                execute!(io::stdout(), cursor::MoveTo(12, current_option + 2));
                println!("Not available\r");
                wait_a_milli(500);
            }
            if current_option == 2 {
                execute!(io::stdout(), cursor::MoveTo(12, current_option + 2));
                println!("Not available\r");
                wait_a_milli(500);
            }
            if current_option == 3 {
                execute!(io::stdout(), cursor::MoveTo(12, current_option + 2));
                new_run();

            }

        }
    }

}
pub fn ask_name() -> io::Result<String> {
    clean();
    disable_raw_mode()?;
    execute!(io::stdout(), cursor::Show)?;
    let player_name = magic::input_on_line("Name yourself");

    enable_raw_mode()?;
    let options = vec!["Return"];
    show_options(&options, 0 as u16);
    return Ok(player_name);
}

pub fn confirm_name(player_name: &String) -> io::Result<bool> {
    let options = vec!["Yes I'm sure.", "Try again"];
    let n: u16 = options.len() as u16;
    let mut current_option = 0;
    let title = format!("Are you sure about that? {}?\n\rI mean I don't really care but..  ", player_name); 
    loop {
        clean();
        magic::simple_line_with_title(&title);
        magic::show_options(&options, current_option);

        let input = crossterm::event::read()?;
        match magic::read_up_down(&input, current_option, n) {
            UpDown::Up => {
                current_option -= 1;
                continue
            },
            UpDown::Down => {
                current_option += 1;
                continue
            },
            UpDown::Nil => {},
        }
        if input == crossterm::event::Event::Key(crossterm::event::KeyCode::Enter.into()) {
            if current_option == 0 {
                break
            }
            if current_option == 1 {
                return Ok(false)
            }

        }
    }
    Ok(true)

}

