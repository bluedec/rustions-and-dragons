use std::thread;
use std::time::Duration;
use std::io::{ self, Write };

use crossterm::event::Event;
use crossterm::{ event, cursor, terminal, execute };
use crossterm::terminal::{ Clear, ClearType };

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


pub fn show_options(options: &Vec<&'static str>, current_option: u16) {
    for (i, option) in options.iter().enumerate() {
        if i == current_option as usize {
            println!("{} <\r", option)
        } else {
            println!("{}\r", option);
        }
    }

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

