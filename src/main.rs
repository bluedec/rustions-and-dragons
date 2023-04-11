use std::io;
use std::io::{ Write, stdout };

use crossterm::{ cursor, execute };
use crossterm::terminal;
use crossterm::terminal::{ enable_raw_mode, disable_raw_mode, Clear, ClearType };
use crossterm::event;
use crossterm::event::Event;

use magic::{ clean, Choice };
use magic::{ Zone, Wing};
use magic::{ read_up_or_down, close };


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut main_menu: Vec<Zone> = Vec::new();
    let start = Zone {
        title: "Rustions and Dragons",
        options: vec![") Start", ") Quit"],
    };
    let new_ran = Zone {
        title: "New Run",
        options: vec![") New Run", ") Return"],   
    };
    let choose_race = Zone {
        title: "Choose Race",
        options: vec![") Humans", ") Locked", ") Locked"],   

    };
    let name = Zone {
        title: "Name yourself.",
        options: vec![],
    };
    
    main_menu.insert(0, start);
    main_menu.insert(0, new_ran);
    main_menu.insert(0, choose_race);
     
    let mut stdout = stdout();
    clean(&mut stdout);

    start_quit(&mut stdout); 
     
    pub fn start_quit(stdout: &mut io::Stdout) {
        let options = vec![") Start", ") Quit"];
        let mut current_option = 0;
        let n = options.len() + 1; 
        loop {

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
            
            // make a function that handles this two cases, Up and Down.
            if input == Event::Key(event::KeyCode::Up.into()) {
                execute!(stdout, Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
                if current_option > 0 {
                    current_option -= 1;
                    continue
                }
            }
            if input == Event::Key(event::KeyCode::Down.into()) {
                execute!(stdout, Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
                if current_option < options.len() - 1 {
                    current_option += 1;
                    continue
                }
            }
            if input == Event::Key(event::KeyCode::Enter.into()) {
                execute!(stdout, Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
                if current_option == 0 {
                    new_run(stdout);
                }
                if current_option == 1 {
                    break 
                }
            }
            execute!(stdout, Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
        }
    }

    pub fn new_run(stdout: &mut io::Stdout) {
        let options = vec![") New Run", ") Return"];
        let mut current_option = 0;
        let n = options.len();
        loop {
            execute!(stdout, Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
            println!("{}", current_option);
            println!(" New Run\r");

            for (i, option) in options.iter().enumerate() {
                if i == current_option as usize {
                    println!("{} <\r", option)
                } else {
                    println!("{}\r", option);
                }
            }
            let input = event::read().unwrap();

 //           match read_up_or_down(stdout, &input, current_option, n) {
 //               Choice::Go => println!("Lol\r"),
 //               Choice::Next => {
 //                   current_option += 1;
 //                   continue
 //               },
 //               Choice::Prev => {
 //                   current_option -= 1;
 //                   continue
 //               },
 //               Choice::Back => {
 //                   start_quit(stdout);
 //               },
 //               Choice::Other => {},
 //           };
            
            if input == Event::Key(event::KeyCode::Up.into()) {
                execute!(stdout, Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
                if current_option > 0 {
                    current_option -= 1;
                    continue
                }
            }
            if input == Event::Key(event::KeyCode::Down.into()) {
                execute!(stdout, Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
                if current_option < ((n as i32) - 1) {
                    current_option += 1;
                    continue
                }
            }
            if input == Event::Key(event::KeyCode::Enter.into()) {
                execute!(stdout, Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
                if current_option == 0 {
                    println!("Nothing Yet!\r");
                }
                if current_option == 1 {
                    start_quit(stdout);
                }

            }
            execute!(stdout, Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
        }

    }
    close(&mut stdout)?;
    Ok(())
}


fn _find_wing() {
    println!("I'm trying to give you a wing here...");
    let _win = Wing {
        name: String::from("WIN"),
        weight: 6,
        habilities: vec![String::from("Gust"), String::from("Heal")],
    };
}
