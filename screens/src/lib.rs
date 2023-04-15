use std::io;
use std::io::Write;
use std::fs;
use std::fs::File;

use crossterm::event::{ Event };
use crossterm::event::{ KeyCode };
use crossterm::{ event, cursor, execute };
use crossterm::terminal::{ Clear, ClearType };
// TODO: Something with these two:
#[allow(unused_imports)]
use crossterm::cursor::{ Show, MoveRight };

use magic::{*, canvas};

use serde_json::*;

pub fn intro() -> io::Result<()> {

    move_to(16, 10);
    let load = "[>--------Tsmhrloz----cxz----Yerhjmw--------<]".to_string();
    for char in load.chars() {
        print!("{}", char);
        io::stdout().flush()?;
        magic::wait_a_milli(100);
    }
    let load = "[>........Rusteons....and....Dragons........<] ".to_string();
    move_to(16, 10);
    for (i, char) in load.chars().enumerate() {
        print!("{}", char);
        if i == load.len() - 1 {
            break;
        } else {
            magic::wait_a_milli(100);
        }
        io::stdout().flush()?;
    }
    print!("Click!");
    io::stdout().flush().unwrap();
    magic::wait_a_milli(750);
    Ok(())
}

pub fn boilerplate() {
    // each item represents a basic necessity for most menus, it's in order.
    // do options for menu
    // default_position (index) of the default option as a u16
    // max value of the current_position (options.len() - 1)
    // coordinates if using show_options_at()
    // loop {
    // magic::show_options_at(&options, default_position)
    // magic::show_options_at(&options, default_position, coordinates)
    // handle events
    // }
    //
}

pub fn start_quit() {
    clean();
    let cs = (26, 26);
    let coordinates = (38, 16);
    canvas_of_size_at(cs, coordinates);
    magic::canvas();
    let mut stdout = io::stdout();
    let mut current_option = 0;
    let options = vec![") Start", ") Quit"];
    let options_len: u16 = options.len() as u16;
    loop {
        title_and_line("Rusteons & Dragons");
        
        magic::show_options_at(&options, current_option, (4, 4));

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
        magic::canvas();
        magic::title_and_line("");
        magic::show_options_at(&options, current_option, (4, 4));

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
        magic::canvas_of_size_at((15, 5), (4, 40));
        magic::canvas_of_size_at((15, 5), (25, 35));
        magic::canvas();
        magic::canvas_of_size_at((15, 5), (65, 15));
        magic::title_and_line(&"What are you?".to_string()); 
        magic::show_options_at(&options, current_option, (4, 4));

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
                move_to(27, 38);
                println!("Not available");
                wait_a_milli(500);
            }
            if current_option == 2 {
                execute!(io::stdout(), cursor::MoveTo(12, current_option + 2)).expect("Execute! failed moving the cursor.");
                move_to(67, 18);
                println!("Not available");
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
        magic::canvas();
        title_and_line("Give yourself a name, or one shall be given to you"); 
        show_options_at(&options, current_option, (4, 4));
         
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
                canvas();
                character_name = input_on_line_at("Give yourself a name", (4, 4));
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

pub fn confirm_name() -> io::Result<bool> {
    let options = vec!["Yes I'm sure.", "Try again"];
    let max: u16 = options.len() as u16;
    let mut current_option = 0;
    let title = "Are you sure about your choice?"; 
    loop {
        clean();
        magic::canvas();
        magic::title_and_line(&title);
        magic::show_options_at(&options, current_option, (4, 4));

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

pub fn create_character() -> io::Result<String> {
    let mut quit = false;
    let mut player_name = String::new();
    let mut player_file: File;
    while quit == false {
        println!("{player_name}");
        player_name = ask_name()?;
        let path_to_file = format!("{}.json", player_name);
        if let Ok(metadata) = fs::metadata(&path_to_file) {
            if metadata.is_file() {
                // TODO: ask if wants to overwrite
                println!("Name taken already by another character.");
                wait_a_sec(1);
                continue
            } else {
                player_file = File::create(&path_to_file).unwrap();
            }
        } else {
            player_file = File::create(&path_to_file).unwrap();
        }
        if let Ok(false) = confirm_name() {
            fs::remove_file(&path_to_file)?;
            continue
        } else {
            quit = true 
        }

    }
    // TODO: Ask class of player 
    // let's do ranger for now.
    let player = Human {
        name: player_name.clone(),
        level: 1,
        health: 100,
        class: Class::Ranger{ level: 1 }, 
    };
    let json_player = serde_json::to_string(&player);  

    Ok(player_name)
}


