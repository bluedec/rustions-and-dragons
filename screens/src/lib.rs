
use std::io;
use std::io::Write;

use crossterm::event::{ Event };
use crossterm::event::{ KeyCode };
use crossterm::{ event, cursor, execute };
use crossterm::terminal::{ Clear, ClearType };
// TODO: Something with these two:
#[allow(unused_imports)]
use crossterm::cursor::{ Show, MoveRight };

use magic::*;

pub fn intro() -> io::Result<()> {
    clean();
    println!("\r");


    std::thread::spawn(move || {
        wait_a_milli(7285);
        print!(" Click!\r");
        io::stdout().flush().expect("Failed flushing stdout.");
    });

    let load = "[>-------------------------------<]".to_string();
    for char in load.chars() {
        print!("{}", char);
        io::stdout().flush()?;
        magic::wait_a_milli(20);
    }

    let mut loaded = String::with_capacity(load.len()); 
    for c in load.chars() {
        loaded.push(
            if c == '-' { '.' } 
            else if c == 'x' { '^' } 
            else if c == 'B' { 'R' } 
            else if c == 't' { 'r' } 
            else if c == 'd' { 'u' } 
            else if c == 'm' { 's' } 
            else if c == 'l' { 't' } 
            else if c == 'y' { 'e' } 
            else if c == 'b' { 'o' } 
            else if c == 's' { 'n' } 
            else if c == 'f' { 's' } 
            else if c == 'g' { 'a' } 
            else if c == 'v' { 'n' } 
            else if c == 'k' { 'D' } 
            else if c == 'z' { 'r' } 
            else if c == 's' { 'a' } 
            else if c == 'w' { 'g' } 
            else if c == 'p' { 'o' } 
            else if c == 'q' { 'd' }  
            else { c });
        print!("\r{}", loaded);
        io::stdout().flush()?;
        magic::wait_a_milli(150);
    }
    magic::wait_a_milli(470);
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
        println!("------------------------\r");

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
                //intro().unwrap();
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
        magic::title_and_line("");
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
        magic::title_and_line(&"What are you?".to_string()); 
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


        if input == Event::Key(KeyCode::Enter.into()) {
            if current_option == 0 {
                break
            }
            if current_option == 1 {
                execute!(io::stdout(), cursor::MoveTo(12, current_option + 2)).expect("Execute! failed moving the cursor.");
                println!("Not available\r");
                wait_a_milli(500);
            }
            if current_option == 2 {
                execute!(io::stdout(), cursor::MoveTo(12, current_option + 2)).expect("Execute! failed moving the cursor.");
                println!("Not available\r");
                wait_a_milli(500);
            }
            if current_option == 3 {
                execute!(io::stdout(), cursor::MoveTo(12, current_option + 2)).expect("Execute! failed moving the cursor.");
                new_run();

            }

        }
    }

}
pub fn ask_name() -> io::Result<String> {
    let from: (u16, u16) = (4, 60);
    clean_up_from(from);

    let mut character_name = String::new();

    let options = vec![") I'll give me one", ") Choose a name for me", "\n) Return"];
    let mut current_option = 0;
    let max: u16 = options.len() as u16;
    
    loop {
    clean_up_from(from);
        title_and_line("Give yourself a name, or one shall be given to you"); 
        show_options_at(&options, current_option, (0, 2));
         
        let input = event::read().expect("Expected input.");
        match magic::read_up_down(&input, current_option, max) {
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
        if input == Event::Key(KeyCode::Enter.into()) {
            if current_option == 0 {
                clean();
                character_name = input_on_line("Give yourself a name");
                return Ok(character_name)

            }
            if current_option == 1 {
                return Ok(String::from("Cappuccino"))
            }
            if current_option == 2 {
                choose_race();
            }
        }
    }

}

pub fn confirm_name(player_name: &String) -> io::Result<bool> {
    let options = vec!["Yes I'm sure.", "Try again"];
    let max: u16 = options.len() as u16;
    let mut current_option = 0;
    let title = format!("Are you sure about that? {}?\n\rI mean I don't really care but..  ", player_name); 
    loop {
        clean();
        magic::title_and_line(&title);
        magic::show_options(&options, current_option);

        let input = event::read()?;
        match magic::read_up_down(&input, current_option, max) {
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
        if input == Event::Key(KeyCode::Enter.into()) {
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

