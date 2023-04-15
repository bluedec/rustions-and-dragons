use std::io;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::sync::{ Arc, Mutex };

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::{ KeyCode::{ Esc }};
use crossterm::execute;
use crossterm::terminal::{ disable_raw_mode, enable_raw_mode };
use crossterm::terminal::{ Clear, ClearType };
use crossterm::cursor::{ self, Hide, Show };

use magic::{self, Human, Class, wait_a_sec, move_to, canvas_of_size_at};

use serde::{ Serialize, Deserialize };
use serde_json::{Result, Value};
use std::io::prelude::*;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    magic::canvas();
    magic::clean();
    //screens::intro()?;
    // TODO: If there is a saved game...
    // TODO: "Would you like to continue?" screen
    // if not {
    screens::start_quit(); 
    screens::new_run();
    screens::choose_race();
    execute!(io::stdout(), crossterm::cursor::Show)?;
    magic::clean();
    let player_name = screens::create_character()?;
    // else { just continue with area where he left

    let player_file_name = format!("{}.json", &player_name);
    let mut file = File::open(player_file_name).expect("1");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("2");
    println!("{}", contents);
    let human_player: Human = serde_json::from_str(&contents).expect("3");
    println!("Your level: {}", human_player.level);

    println!("Getting 's info..");

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    //screens::intro()?
    magic::close()?;
    Ok(())
}


