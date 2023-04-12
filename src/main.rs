use std::io;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::path::Path;

use crossterm::execute;
use crossterm::terminal::{ disable_raw_mode, enable_raw_mode };
use crossterm::cursor::Hide;

use magic::{ clean, close, show_options, UpDown };


fn main() -> io::Result<()> {
    clean();
    execute!(io::stdout(), Hide).unwrap();
    
    let load = "[>------------------------------------------<]".to_string();
    for char in load.chars() {
        print!("{}", char);
        io::stdout().flush().unwrap();
        magic::wait_a_milli(20);
    }
    let mut loaded = String::with_capacity(load.len()); 
    for c in load.chars() {
        loaded.push(if c == '-' { '#' } else { c });
        print!("\r{}", loaded);
        io::stdout().flush().unwrap();
        magic::wait_a_milli(50);
    }
    magic::wait_a_milli(10);
    println!(" Click!\r");
    magic::wait_a_milli(220);
    enable_raw_mode()?;

    let user_data_path = Path::new("user_data.txt");
    let user_data = File::create("user_data.txt");

    magic::wait_a_milli(300);

    clean();

    screens::start_quit(); 
    screens::new_run();
    screens::choose_race();

    
    get_and_confirm_name()?; 

    pub fn get_and_confirm_name() -> io::Result<()> {
        println!("\nName yourself\r");
        execute!(io::stdout(), crossterm::cursor::Show)?;
        disable_raw_mode()?;

        let player_name = magic::take_inp();

        enable_raw_mode()?;
        let options = vec!["Yes", "No"];
        let n: u16 = options.len() as u16;
        let mut current_option = 0;
        loop {
            clean();
            println!("Is '{}' ok?", player_name.trim());
            magic::show_options(&options, current_option);
            let input = crossterm::event::read()?;
            match magic::read_up_down(&input, current_option, n) {
                UpDown::Down => {
                    current_option += 1;
                    continue
                },
                UpDown::Up => {
                    current_option -= 1;
                    continue
                },
                UpDown::Nil => {},
            }
            if input == crossterm::event::Event::Key(crossterm::event::KeyCode::Enter.into()) {
                if current_option == 0 {
                    break
                }
                if current_option == 1 {
                   break 
                }

            }
        }
        Ok(())

    }

    enable_raw_mode()?;

    close()?;
    Ok(())
}


