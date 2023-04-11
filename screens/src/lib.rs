use std::io;


use crossterm::event::Event;
use crossterm::{ event, cursor, terminal, execute };
use crossterm::terminal::{ Clear, ClearType };

use magic::*;

pub fn start_quit() {
    let mut stdout = io::stdout();
    let mut current_option = 0;
    let options = vec![") Start", ") Quit"];
    loop {
        clean();
        println!("{}", current_option);
        println!(" Main Menu\r");

        for (i, option) in options.iter().enumerate() {
            if i == current_option as usize {
                println!("{} <\r", option)
            } else {
                println!("{}\r", option);
            }
        }
        let input = event::read().unwrap();

        match read_up_down(&input, current_option, options.len()) {
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
                new_run();
            }
            if current_option == 1 {
                close().unwrap();
            }
        }
        clean();
    }
}


pub fn new_run() {
    let options = vec![") New Run", ") Return"];
    let mut current_option = 0;
    let n = options.len();
    loop {
        clean();
        println!("{}", current_option);
        println!("\r");

        for (i, option) in options.iter().enumerate() {
            if i == current_option as usize {
                println!("{} <\r", option)
            } else {
                println!("{}\r", option);
            }
        }
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
                choose_race();
            }
            if current_option == 1 {
                start_quit();
            }

        }
        clean();
    }

}

pub fn choose_race() {
    let options = vec![") Human", ") Blocked", ") Blocked"];
    let mut current_option = 0;
    loop {
        clean();
        println!("{}", current_option);
        println!("\r");

        for (i, option) in options.iter().enumerate() {
            if i == current_option as usize {
                println!("{} <\r", option)
            } else {
                println!("{}\r", option);
            }
        }
        let input = event::read().unwrap();

        match read_up_down(&input, current_option, options.len()) {
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
                println!("Nothing Yet!\r");
            }
            if current_option == 1 {
                start_quit();
            }

        }
        clean()
    }

}

