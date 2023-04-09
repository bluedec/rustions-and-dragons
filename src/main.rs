use std::sync::mpsc;
use std::thread;
use std::time::Duration; 
use std::io;
use magic::Class;
use std::io::{Write, stdout, stdin};
use crossterm::event::Event;
use crossterm::{
    event, QueueableCommand, cursor, ExecutableCommand, queue
};
use crossterm::terminal;
use crossterm::cursor::{ Show, Hide };
use crossterm::{ execute, terminal::{ Clear, ClearType } };

pub fn take_inp() -> String {
    let mut input2 = String::new();
    stdin()
        .read_line(&mut input2)
        .expect("Failed to read input.");

    input2.trim().to_lowercase().to_string()
}


fn main() {
    let mut stdout = stdout();
    execute!(stdout, Hide).unwrap();
    execute!(stdout, Clear(ClearType::All)).unwrap(); 
    execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
    let dragon = "                                                                      
                                 d$                                          
                               .$$F                                          
                              z$$$                             d$$b     .$$  
-__                          J$$$$                            d$. $$$$$$$$$  
  \\                         4$$$$$                           z$$$$$$$$$$$$\"  
   \"c                       $$$$$$F                         d$$$*$$$$\"       
    \"$e.      .            $$$$$$$$                       .$$$$$             
     3$$$$$$P\"             $$$$$$$$c                    .$$$$$$              
      $$$$$\"               $$$$$$$$$r                .e$$$$$$$F              
      *$$$b                $$$$$$$$$$c           .e$$$$$$$$$$P               
      '$P \"$.              $$$$$$$$$$$b.    .ee$$$$$$$$$$$$$P                
       $   ^$$.             $$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$F                 
       \"     $$$$e..      .e$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$\"                  
             \"$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$P                    
                *$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$P                      
                  \"$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$*                        
                     \"*$$$$$$$$$$$$$$$$$$$$$$$$$$$\"                          
                         \"*$$$$$$$$$$$$$$$$$$$$$\"                            
                             *$$$$$$$$$$$$$$$\"                               
                                  \"\"\"\"\"\"\"\"\"       Gilo94'";
    println!("{}", dragon);
    println!("\nWelcome to Rustions and Dragons");        

    let mut options: Vec<&str> = vec![") Start", ") Quit"];
    let mut selected_option: i32 = 0;
    let max_option_range = options.len() - 1;
    // transmiter for input
    let (tx, rx): (mpsc::Sender<bool>, mpsc::Receiver<bool>)      = mpsc::channel();
    
    pub fn show_options(options: &Vec<&str>, selected_option: i32) {
        for (i, option) in options.iter().enumerate() {
            if i == selected_option.try_into().unwrap() {
                println!("{} <", option)
            } else {
                println!("{}", option);

            }
        }
    }
    show_options(&options, selected_option);

    // thread made to receive input 
    let input_thread = thread::spawn(move || {
        // raw mode for receive instantly the user input
        loop {
            println!("Waiting for input.");
            let event = event::read().unwrap();
            if event == Event::Key(event::KeyCode::Up.into()) {
                if selected_option > 0 {
                    tx.send(true).unwrap();
                    break
                }
            }
            if event == Event::Key(event::KeyCode::Down.into()) {
                if selected_option < max_option_range.try_into().unwrap() {
                    tx.send(false).unwrap();
                }
            }
            if event == Event::Key(event::KeyCode::Esc.into()) {
                break
            }
        }
        terminal::disable_raw_mode().unwrap();
    }).join();

    let data = rx.recv().unwrap();
    if data == true {
        selected_option -= 1;
    } else {
        selected_option += 1;
    }
    for (i, option) in options.iter().enumerate() {
        if i == selected_option.try_into().unwrap() {
            println!("{} <", option);
        } else {
            println!("{}", option);
        }
    }
    println!("{:?}", data);



    stdout.flush().unwrap();
}


