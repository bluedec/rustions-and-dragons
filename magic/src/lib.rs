use std::thread;
use std::time::Duration;
use std::io;

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
    Break,
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

pub fn wait_a_sec() {
    thread::sleep(Duration::from_secs(1));
}



pub fn zone(zone: &Zone, selected_option: i32) -> Choice {
    // zone takes care of calling the show option method on the zone, let's you choose the default
    // option and takes a transmiter to send data.
    let n = zone.options.len() + 1;
    let max_option_range = zone.options.len() - 1;
    let mut so = selected_option.clone();

    loop {
        // we pass the "selected option" to print the currently selected option
        zone.show_options(so); 

        // wait for key press
        let event = event::read().unwrap();

        if event == Event::Key(event::KeyCode::Up.into()) {
            execute!(io::stdout(), Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap(); 
            if so > 0 {
                so -= 1;
                continue
            }
        }

        if event == Event::Key(event::KeyCode::Enter.into()) {
            execute!(io::stdout(), Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
            return Choice::Go
        }

        if event == Event::Key(event::KeyCode::Down.into()) {
            execute!(io::stdout(), Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
            if so < max_option_range as i32 {
                so += 1;
            }                     
        }

        if event == Event::Key(event::KeyCode::Esc.into()) {
            // might add a pop up screen to confirm the exit
            println!("Exiting the listen...");
            wait_a_sec();
            return Choice::Back
        }

            execute!(io::stdout(), Clear(ClearType::All), cursor::MoveUp(n as u16)).unwrap();
        
        
    }
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

