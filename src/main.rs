use std::io;
use std::io::{Write, stdout };

use std::sync::mpsc;

use crossterm::event::Event;
use crossterm::{ event, cursor, terminal, execute };
use crossterm::terminal::{ enable_raw_mode, disable_raw_mode, Clear, ClearType };

use magic::{ wait_a_sec, Choice };
use magic:: show_options;



fn main() -> io::Result<()> {
    let mut stdout = stdout();
    execute!(
        stdout,
        cursor::Hide,
        cursor::MoveTo(0, 0),
        Clear(ClearType::CurrentLine),
    ).unwrap();

    enable_raw_mode()?;

    let options: Vec<&str> = vec![") Start", ") Quit"];
    let mut selected_option: i32 = 0;
    let max_option_range = options.len() - 1;

    pub fn zone(options: Vec<&str>, selected_option: i32, title: &str) {
        'zone: loop {
            show_options(&options, selected_option, title);
            let event = event::read().unwrap();
        }
    }
    'master: loop {
        execute!(stdout, Clear(ClearType::All)).unwrap();
        // transmiter for input
        let (tx, rx): (mpsc::Sender<Choice>, mpsc::Receiver<Choice>) = mpsc::channel();

        show_options(&options, selected_option, "Main Menu\r");

        // cloned for simplicity
        let slc_opt_copy = selected_option.clone();
        let max_option_range_copy = max_option_range.clone();

        // thread made to receive input 
        let _input_thread = std::thread::spawn(move || {
            loop {
                let event = event::read().unwrap();

                if event == Event::Key(event::KeyCode::Up.into()) {
                    if slc_opt_copy > 0 {
                        execute!(io::stdout(), Clear(ClearType::FromCursorUp), cursor::MoveUp(3)).unwrap();
                        tx.send(Choice::Next).expect("Lol 1");
                        break;
                    }
                }

                if event == Event::Key(event::KeyCode::Enter.into()) {
                    
                }

                if event == Event::Key(event::KeyCode::Down.into()) {
                    if slc_opt_copy < max_option_range_copy.try_into().unwrap() {
                        execute!(io::stdout(), Clear(ClearType::FromCursorUp), cursor::MoveUp(3)).unwrap();
                        tx.send(Choice::Prev).expect("Lol 2");
                        break
                    }                     
                }

                if event == Event::Key(event::KeyCode::Esc.into()) {
                    // might add a pop up screen to confirm the exit
                    terminal::Clear(ClearType::FromCursorUp);
                    println!("Exiting the listen...\r");
                    wait_a_sec();
                    tx.send(Choice::Break).expect("Failed to send Choice::Break");
                    break
                }

            }
        });

        let data = rx.recv().unwrap();

        match data {
            Choice::Next => selected_option -= 1,
            Choice::Prev => selected_option += 1,
            Choice::Break => break 'master

        }


    }
    println!("Disabling Raw Mode\r");

    execute!(stdout, cursor::Show)?;
    stdout.flush().unwrap();

    disable_raw_mode()
}


