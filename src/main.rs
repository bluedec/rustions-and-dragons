use std::thread;
use std::time::Duration; 
use magic::Class;
use std::io::{Write, stdout, stdin};
use crossterm::{
    event, QueueableCommand, cursor, ExecutableCommand, queue
};
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
    let mut stdout = stdout();
    println!("\n<<<0/ O \\0>>>Welcome to Rustions and Dragons");        
    println!(") Start");
    println!(") Quit");
    execute!(stdout, Hide).unwrap();
    stdout.queue(cursor::MoveTo(65, 28)).unwrap();
     
    execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
    println!("Your cursor here.");
    stdout.flush();
}


