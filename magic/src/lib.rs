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
    Break,
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

pub fn zone(options: &Vec<&str>, selected_option: i32, title: &str, tx: &Arc<Mutex<mpsc::Sender<Choice>>>) {

    let max_option_range = options.len() - 1;
    let mut so = selected_option.clone();
    let n = options.len() + 1;

    loop {

        println!(" {}\r", title);

        for (i, option) in options.iter().enumerate() {
            if i == so as usize {
                println!("{} <\r", option)
            } else {
                println!("{}\r", option);

            }
        }

        let event = event::read().unwrap();

        if event == Event::Key(event::KeyCode::Up.into()) {
            execute!(io::stdout(), Clear(ClearType::FromCursorUp), cursor::MoveUp(n as u16)).unwrap(); 
            if so > 0 {
                so -= 1;
                continue
            }
        }

        if event == Event::Key(event::KeyCode::Enter.into()) {
            execute!(io::stdout(), Clear(ClearType::FromCursorUp), cursor::MoveUp(n as u16)).unwrap();
            tx.lock().unwrap().send(Choice::Go).unwrap();
        }

        if event == Event::Key(event::KeyCode::Down.into()) {
            execute!(io::stdout(), Clear(ClearType::FromCursorUp), cursor::MoveUp(n as u16)).unwrap();
            if so < max_option_range as i32 {
                so += 1;
                continue
            }                     
        }

        if event == Event::Key(event::KeyCode::Esc.into()) {
            // might add a pop up screen to confirm the exit
            terminal::Clear(ClearType::FromCursorUp);
            println!("Exiting the listen...\r");
            wait_a_sec();
            //tx.send(Choice::Break).expect("Failed to send Choice::Break");
            break
        }
    }
}
pub fn take_inp() -> String {
    let mut input2 = String::new();
    std::io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input.");

    input2.trim().to_lowercase().to_string()
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

