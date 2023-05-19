
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

pub use timer::Clock;
pub use modes::Event;


pub fn io_thread(tx: Sender<Event>){
    let mut state = AppState{
        sender    : tx,
        mode      : Mode::Normal,

        prev_name : String::from(""),
        selection : 0,
        timers    : vec![],
        path      : String::from(shellexpand::tilde( "~/.cache/timah/")),

        timer     : Timer{
            view     : TimeFrame::Total,
            state    : TimerState::Paused,
            name     : String::from(""),
            now      : SystemTime::now(),
            interval : Duration::from_secs(1),
            start    : Local::now(),
            times    : Times::default(),
            total    : Times::default(),
            mode     : TimerAccumulate::Timer,
        },
    };

    state.sender.send(Event::Init{
        timer       : state.timer.name.clone(),
        timeframe   : state.timer.get_view(),
        clock       : state.timer.get_clock(),
        legend      : keybinds::get_legend(&state.mode),
    }).unwrap();

    loop {
        let c = getch(); 

        // mode independent inputs
        if c == KEY_RESIZE {
            state.sender.send(Event::Resize).unwrap();
        }

        let action = keybinds::get_action(&state.mode, c);
        if let Control::Break = handle_action(&mut state, action){
            break;
        }

        timer(&mut state);

        napms(10);
    }
}
