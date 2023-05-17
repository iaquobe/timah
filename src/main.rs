use std::thread;
use std::env;
use std::sync::mpsc::{self, channel};
use ncurses::*;

mod io; 
mod ui; 

use io::*;
use ui::*;



fn main() {
    // for quick escape key
    env::set_var("ESCDELAY", "0");
    setlocale(LcCategory::all, "");

    // Initialize ncurses
    initscr();
    cbreak();
    raw();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // don't wait for input from user
    keypad(stdscr(), true);
    nodelay(stdscr(), true); 

    let (tx, rx): (mpsc::Sender<Event>, mpsc::Receiver<Event>) = channel();

    let io_thread    = thread::spawn(move|| {io_thread(tx)}); 
    let ui_thread    = thread::spawn(|| {ui_thread(rx)}); 

    ui_thread.join().expect("could not join ui thread");
    io_thread.join().expect("could not join io thread");

    endwin();
}

