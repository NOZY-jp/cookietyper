use std::{io::stdin, thread};

use cookietyper_core::Game;
fn main() {
    let mut game = Game::default();

    let (tx, rx) = flume::bounded(5);

    thread::spawn(move || {
        loop {
            let mut s = String::new();

            if let Err(e) = stdin().read_line(&mut s) {
                eprintln!("{e}");
            }

            let _ = tx.send(s);
        }
    });

    loop {
        if let Ok(s) = rx.try_recv() {
            println!("{s}")
        }
    }
}
