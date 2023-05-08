
use std::time::{Duration,SystemTime};
use std::sync::mpsc::Sender;

use chrono::prelude::*;
use ncurses::*;
use shellexpand;

mod files;
mod keybinds;
mod modes;
mod timer;

use keybinds::Mode;
use modes::*;
use timer::*;

pub use timer::Times;
pub use modes::Event;


pub fn io_thread(tx: Sender<Event>){
    let mut state = AppState{
        mode      : Mode::Normal,

        prev_name : String::from(""),
        selection : 0,
        timers    : vec![],
        path      : String::from(shellexpand::tilde( "~/.cache/taimah/")),

        timer     : Timer{
            state    : TimerState::Paused,
            name     : String::from(""),
            now      : SystemTime::now(),
            interval : Duration::from_secs(1),
            start    : Local::now(),
            seconds  : 0,
        },
    };


    tx.send(Event::Init(state.timer.name.clone(), Times::from(state.timer.seconds))).unwrap();

    loop {
        let c = getch(); 

        // mode independent inputs
        if c == KEY_RESIZE {
            tx.send(Event::Resize).unwrap();
        }

        let action = keybinds::get_action(&state.mode, c);
        if let Control::Break = handle_action(&tx, &mut state, action){
            break;
        }

        timer(&tx, &mut state);

        napms(10);
    }
}
