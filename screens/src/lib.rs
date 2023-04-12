use std::io;

use crossterm::event::Event;
use crossterm::{ event, cursor, execute };
use crossterm::terminal::{ Clear, ClearType };

use magic::*;

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
    let n: u16 = options.len() as u16;
    loop {
        clean();
        println!("\r");
        println!("-------------------------------\r");

        magic::show_options(&options, current_option);

        let input = event::read().unwrap();

        match read_up_down(&input, current_option, n) {
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
        println!("What are you?\r");
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

