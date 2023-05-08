use std::sync::mpsc::Sender;
use std::time::{Duration,SystemTime};
use chrono::prelude::*;
use crate::io::{AppState,Event};

pub enum TimerState {
    Paused, 
    Running,
}

pub struct Times {
    pub seconds :i32,
    pub minutes :i32,
    pub hours   :i32
}

impl From<i32> for Times {
    fn from(seconds: i32) -> Self {
        let mut minutes = seconds / 60;
        let     hours   = minutes / 60;

        minutes = minutes % 60;
        let     seconds = seconds % 60;

        Times{seconds, minutes, hours}
    }
}

pub struct Timer {
    pub state   :TimerState,
    pub name    :String,
    pub seconds :i32,
    pub now         :SystemTime,
    pub interval    :Duration,
    pub start       :DateTime<Local>,
}


pub fn timer(tx: &Sender<Event>, state: &mut AppState) {
    // check if 1 second has elapsed
    if state.timer.now.elapsed().unwrap() >= state.timer.interval {
        if let TimerState::Running = state.timer.state {
            state.timer.seconds += 1; 

            tx.send(Event::Tick(Times::from(state.timer.seconds))).unwrap(); 
        }
        state.timer.now = state.timer.now + state.timer.interval; 
    }
}

