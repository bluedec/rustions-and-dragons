use std::thread;
use std::time::Duration;
use std::io::{ self, Write };
use std::sync::mpsc::Sender;

use crossterm::event::Event;
use crossterm::{ event, cursor, terminal, execute };
use crossterm::terminal::{ Clear, ClearType, enable_raw_mode };

pub enum UpDown {
    Up,
    Down,
    Nil
}

pub struct Human {
    pub name: String,
    pub level: u16,
    pub health: i32,
    pub class: Class,
}

pub struct Class {
    pub name: String,
    pub level: u16,
}

pub struct Weapon {
    pub name: String,
    pub level: u16,  
    pub damage: i32,
    pub class: Class,
    pub min_lvl: u16, 
}

pub struct Wing {
    pub name: String,
    pub weight: usize,
    pub habilities: Vec<String>,
}

pub fn input_on_line(output: &str) -> String {
    let mut current_option = 0;
    let options = vec!["Return"]; 
    let coordinates = (3, 30);
    let max: u16 = options.len() as u16;
    loop {
        show_options_at(&options, current_option, coordinates);
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
            }

        }
    }


    let mut input = String::new();
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, 1)).unwrap();
    println!("-------------------------------");
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();



    print!("{}: ", output);
    io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input.trim().to_string()

}

pub fn simple_line_with_title(title: &str) {
    println!("{title}\r");
    println!("-------------------------------\r"); 
}

pub fn show_options(options: &Vec<&'static str>, current_option: u16) {
    for (i, option) in options.iter().enumerate() {
        if i == current_option as usize {
            println!("{} <\r", option)
        } else {
            println!("{}\r", option);
        }
    }

}
pub fn show_options_at(options: &Vec<&'static str>, current_option: u16, at: (u16, u16)) {
    cursor::MoveTo(at.0, at.1);
}

pub fn read_up_down(input: &Event, current_option: u16, max: u16) -> UpDown {
    if input == &Event::Key(event::KeyCode::Up.into()) {
        clean();
        if current_option > 0 {
            return UpDown::Up 
        }
    }
    if input == &Event::Key(event::KeyCode::Down.into()) {
        clean();
        if current_option < (max - 1) {
            return UpDown::Down 
        }
    }
    return UpDown::Nil 
}

pub fn wait_a_sec(sec: u64) {
    thread::sleep(Duration::from_secs(sec));
}
pub fn wait_a_milli(milli: u64) {
    thread::sleep(Duration::from_millis(milli));
}

pub fn clean() {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0),
        ).unwrap();
}

pub fn esc_listening_thread() {
    std::thread::spawn(||{
        loop {
            wait_a_milli(50);
            let press = event::read().unwrap();   
            if press == event::Event::Key(event::KeyCode::Esc.into()) {
                close().unwrap();
            }

        }
    });

}


pub fn take_inp() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input.trim().to_string()
}

pub fn close() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    println!("Showing cursor\r");
    execute!(stdout, cursor::Show)?;
    println!("Disabling Raw Mode\r");
    crossterm::terminal::disable_raw_mode()?;
    io::stdout().flush()?;
    println!("Exiting..");
    std::process::exit(999);
}


pub fn dragon_intro() {

    let dragon = "                                                                      
                                 d$                                          
                               .$$F                                          
                              z$$$                             d$$b     .$$  
-__                          J$$$$                            d$. $$$$$$$$$  
  \\                         4$$$$$                           z$$$$$$$$$$$$\"  
   \"c                       $$$$$$F                         d$$$*$$$$\"       
    \"$e.      .            $$$$$$$$                       .$$$$$\r               
     3$$$$$$P\"             $$$$$$$$c                    .$$$$$$\r             
      $$$$$\"               $$$$$$$$$r                .e$$$$$$$F\r              
      *$$$b                $$$$$$$$$$c           .e$$$$$$$$$$P\r               
      '$P \"$.              $$$$$$$$$$$b.    .ee$$$$$$$$$$$$$P\r                
       $   ^$$.             $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$F\r                 
       \"     $$$$e..      .e$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$\"\r                  
             \"$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$P\r                    
                *$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$P \r                      
                  \"$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$*\r                       
                     \"*$$$$$$$$$$$$$$$$$$$$$$$$$$$\"\r                          
                         \"*$$$$$$$$$$$$$$$$$$$$$\"\r                            
                             *$$$$$$$$$$$$$$$\"\r                               
                                  \"\"\"\"\"\"\"\"\"       Gilo94'";
    println!("{}", dragon);
    println!("\nWelcome to Rustions and Dragons\r");        
}

