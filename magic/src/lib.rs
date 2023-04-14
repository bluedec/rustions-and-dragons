use std::thread;
use std::time::Duration;
use std::io::{ self, Write };

use crossterm::event::Event;
use crossterm::{ event, cursor, terminal, execute };
use crossterm::terminal::{ Clear, ClearType };
use crossterm::cursor::{ SavePosition, RestorePosition, MoveRight };

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


pub fn square_intro(height: u16, width: u16) {
    for row in 0..width {
        if row > 50 {
            wait_a_milli(15);
            print!("-");
            io::stdout().flush();
            
        } else {
            print!(" ");
            continue
        }
    }
    for col in 0..height {
        wait_a_milli(15);
        execute!(io::stdout(), cursor::MoveTo(width, col + 1));
        println!("|");
        io::stdout().flush();
    }
    for col in 0..height {
    }
    execute!(io::stdout(), cursor::MoveTo(0, 0));

}
pub fn square(height: u16, width: u16) {
    for row in 0..width {
        if row > 50 {
            print!("-");
            io::stdout().flush();
            
        } else {
            print!(" ");
            continue
        }
    }
    for col in 0..height {
        execute!(io::stdout(), cursor::MoveTo(width, col + 1));
        println!("|");
        io::stdout().flush();
    }
    for col in 0..height {
    }
    execute!(io::stdout(), cursor::MoveTo(0, 0));

}

pub fn intro2(height: u16, width: u16) {
    let one = thread::spawn(move || {
        let mut w_counter = 0;
        let mut h_counter = 0;
        loop {
            wait_a_milli(1);
            print!(".");
            io::stdout().flush();
            w_counter += 1;
            if w_counter > width {
                println!("\r");
                w_counter = 0;
                h_counter += 1;
            }
            if h_counter > height {
                break
            }
        }
    });
    one.join();
}


// todo
pub fn show_text_slowly_at(coordinates: (u16, u16)) {

}

// NOTE: lines should be 24 digits long.
pub fn line_at(coordinates: (u16, u16)) {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::SavePosition, cursor::MoveTo(coordinates.0, coordinates.1)).unwrap();
    println!("------------------------");
    execute!(stdout, cursor::RestorePosition).unwrap();
}


pub fn print_at(text: &'static str, coordinates: (u16, u16)) {
    execute!(io::stdout(), cursor::SavePosition, cursor::MoveTo(coordinates.0, coordinates.1)).unwrap();
    println!("{text}\r");
    execute!(io::stdout(), cursor::RestorePosition).unwrap();
}


pub fn title_and_line(title: &str) {
    println!("{title}\r");
    println!("------------------------\r"); 
}

pub fn input_on_line(output: &str) -> String {
    terminal::disable_raw_mode().expect("Disabling of raw mode failed.");
    let mut input = String::new();
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, 1)).unwrap();
    println!("------------------------");
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

    print!("{}: ", output);
    io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    terminal::enable_raw_mode().expect("Enabling of raw mode failed.");
    input.trim().to_string()

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
    execute!(io::stdout(), cursor::MoveTo(at.0, at.1)).unwrap();
    for (i, option) in options.iter().enumerate() {
        if i == current_option as usize {
            println!("{} <\r", option)
        } else {
            println!("{}\r", option);
        }
    }
    execute!(io::stdout(), cursor::MoveTo(0, 1)).unwrap();
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

pub fn clean_up_from(coordinates: (u16, u16)) {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        cursor::MoveTo(coordinates.0, coordinates.1),
        Clear(ClearType::FromCursorUp),
        cursor::Hide,
        cursor::MoveTo(0, 0),
        ).unwrap();
}

pub fn clean() {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        cursor::MoveTo(40, 15),
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

