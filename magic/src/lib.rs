use std::thread;
use std::time::Duration;
use std::io::{ self, Write };

use crossterm::event::Event;
use crossterm::{ event, cursor, terminal, execute };
use crossterm::terminal::{ Clear, ClearType, disable_raw_mode, enable_raw_mode };
use crossterm::cursor::{ SavePosition, RestorePosition, MoveRight };

use serde::{ Serialize, Deserialize };
use serde_derive::*;


pub enum UpDown {
    Up,
    Down,
    Nil
}
#[derive(Serialize, Deserialize)]
pub struct Human {
    pub name: String,
    pub level: u16,
    pub health: i32,
    pub class: Class,
}

#[derive(Serialize, Deserialize)]
pub enum Class {
    Ranger { level: u16 },
    Warrior { level: u16 },
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

pub fn move_to(x: u16, y: u16) {
    execute!(io::stdout(), cursor::MoveTo(x, y)).unwrap();
}


pub fn canvas_of_size_at(size: (u16, u16), coordinates: (u16, u16)) {
    let mut x = coordinates.0;
    let mut y = coordinates.1;
    let columns = size.0;
    let rows = size.1;
    let mut stdout = io::stdout();

    move_to(x, y);
    print!(" ");
    stdout.flush().unwrap();
    for _ in 0..columns {
        x += 1;
        move_to(x, y);
        print!("_");
    }
    x += 1;
    stdout.flush();
    for r in 0..rows {
        y += 1;
        move_to(x, y);
        print!("|");
    }
    y += 1;
    move_to(x, y);
    print!("°");
    stdout.flush();
    for _ in 0..columns {
        x -= 1;
        move_to(x, y);
        print!("-");
    }
    x -= 1;
    move_to(x, y);
    print!("°");
    stdout.flush();
    move_to(x, y);
    for _ in 0..rows {
        y -= 1;
        move_to(x, y);
        print!("|")
    }
    move_to(0, 0);
}

pub fn canvas() {
    let (mut columns, mut rows) = crossterm::terminal::size().unwrap();
    columns -= 3;
    rows -= 3;

    print!(" ");
    for _ in 0..columns {
        print!("_");
    }
    print!("  ");
    for r in 0..rows {
        print!("|\n\r");
        move_to(columns + 1, r + 1);
        print!("|\n\r");
        
    }
    print!("°");
    for _ in 0..columns {
        print!("-");
    }
    print!("°");
}

pub fn intro2(columns: u16, rows: u16) {
    let one = thread::spawn(move || {
        let mut w_counter = 0;
        let mut h_counter = 0;
        loop {
            wait_a_milli(1);
            print!(".");
            io::stdout().flush();
            w_counter += 1;
            if w_counter > rows {
                println!("\r");
                w_counter = 0;
                h_counter += 1;
            }
            if h_counter > columns {
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
    move_to(2, 2);
    println!("{title}\r");
    move_to(2, 3);
    println!("------------------------\r"); 
}

pub fn input_on_line_at(output: &str, coordinates: (u16, u16)) -> String {
    let col = coordinates.0;
    let row = coordinates.1;
    let mut input = String::new();
    let mut stdout = io::stdout();

    // we disable raw mode to allow input.
    terminal::disable_raw_mode().expect("Disabling of raw mode failed.");
    execute!(stdout, crossterm::cursor::Show).unwrap();
    move_to(col, row - 1);
    println!("------------------------");
    move_to(col, row);
    print!("{}: ", output);
    io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    // enable it back
    terminal::enable_raw_mode().expect("Enabling of raw mode failed.");
    input.trim().to_string()

}

pub fn show_options_at(options: &Vec<&'static str>, current_option: u16, coordinates: (u16, u16)) {
    let mut col = coordinates.0;
    let mut row = coordinates.1;
    let mut stdout = io::stdout();
    for (i, option) in options.iter().enumerate() {
        move_to(col, row); 
        row += 1;
        if i == current_option as usize {
            print!("{} <- ", option);
            stdout.flush().unwrap();

        } else {
            print!("{}   ", option);
            stdout.flush().unwrap();
        }
    }

}

pub fn read_up_down(input: &Event, current_option: u16, max: u16) -> UpDown {
    if input == &Event::Key(event::KeyCode::Up.into()) {
        if current_option > 0 {
            return UpDown::Up 
        }
    }
    if input == &Event::Key(event::KeyCode::Down.into()) {
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
    let col = coordinates.0;
    let row = coordinates.1;
    execute!(
        stdout,
        cursor::MoveTo(col, row),
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

