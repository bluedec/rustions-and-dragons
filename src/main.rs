use std::io;
use std::io::{ Write, stdout };

use std::sync::{ Arc, Mutex };

use std::sync::mpsc;

use crossterm::{ cursor, execute };
use crossterm::terminal::{ enable_raw_mode, disable_raw_mode, Clear, ClearType };

use magic::{ Choice };
use magic::{ zone, Wing};



fn main() -> io::Result<()> {
    let mut stdout = stdout();
    pub fn clean(stdout: &mut io::Stdout) {
        execute!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
            ).unwrap();
    }
    clean(&mut stdout);
    println!("Enabling Raw Mode\r");
    enable_raw_mode()?;

    let options: Vec<&str> = vec![") Start", ") Quit"];

    'master: loop {
        // transmiter for input
        let (tx, rx): (mpsc::Sender<Choice>, mpsc::Receiver<Choice>) = mpsc::channel();

        let tx_arc: Arc<Mutex<mpsc::Sender<Choice>>> = Arc::new(Mutex::new(tx));
        let tx_arc_clone = tx_arc.clone();
        
        zone(&options, 0, "Main Menu", &tx_arc_clone); 

        let data = rx.recv().unwrap();

        match data {
            Choice::Go => println!("Go!"),
            Choice::Next => println!("+1?"),
            Choice::Prev => println!("-1?"),
            Choice::Break => break 'master

        }
    }

    execute!(stdout, cursor::Show)?;
    println!("Disabling Raw Mode\r");
    stdout.flush().unwrap();
    disable_raw_mode()

}


fn _find_wing() {
    println!("I'm trying to give you a wing here...");
    let _win = Wing {
        name: String::from("WIN"),
        weight: 6,
        habilities: vec![String::from("Gust"), String::from("Heal")],
    };
}
