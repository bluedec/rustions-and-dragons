use std::thread;
use std::time::Duration;
use std::io::{ self, Write };

use std::sync::mpsc;
use std::sync::{ Arc, Mutex };

use crossterm::event::Event;
use crossterm::{ event, cursor, terminal, execute };
use crossterm::terminal::{ Clear, ClearType };

pub enum Choice {
    Go,
    Next,
    Prev,
    Back,
    Other,
}

pub struct Zone<'a> {
    pub title: &'a str,
    pub options: Vec<&'static str>,
}

impl<'a> Zone<'a> {
    pub fn show_options(&self, so: i32) {
        println!(" {}\r", self.title);

        for (i, option) in self.options.iter().enumerate() {
            if i == so as usize {
                println!("{} <\r", option)
            } else {
                println!("{}\r", option);

            }
        }

    }
}

pub struct Human {
    pub name: String,
    pub health: i32,
    pub class: Class,
}
pub struct Class {
    pub name: String,
    pub level: i32,
}
pub struct Weapon {
    pub name: String,
    pub level: i32,  
    pub damage: i32,
    pub class: Class,
    pub min_lvl: i32, 
}

pub struct Wing {
    pub name: String,
    pub weight: usize,
    pub habilities: Vec<String>,
}

pub fn read_up_or_down(stdout: &mut io::Stdout, input: &Event, current_option: i32, n: usize) -> Choice {
    if input == &Event::Key(event::KeyCode::Up.into()) {
        execute!(stdout, Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
        if current_option > 0 {
            return Choice::Prev
        }
    }
    if input == &Event::Key(event::KeyCode::Down.into()) {
        execute!(stdout, Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
        if current_option < n as i32 {
            return Choice::Next
        }
    }
    Choice::Other
}

pub fn wait_a_sec() {
    thread::sleep(Duration::from_secs(1));
}

pub fn clean(stdout: &mut io::Stdout) {
    execute!(
        stdout,
        cursor::Hide,
        cursor::MoveTo(0, 0),
        Clear(ClearType::All),
        ).unwrap();
}

pub fn take_inp() -> String {
    let mut input2 = String::new();
    std::io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input.");

    input2.trim().to_lowercase()
}

pub fn close(stdout: &mut io::Stdout) -> Result<(), io::Error> {
    execute!(stdout, cursor::Show)?;
    println!("Disabling Raw Mode\r");
    io::stdout().flush()?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
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

