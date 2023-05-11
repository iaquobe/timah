use std::sync::mpsc::Sender;
use std::time::{Duration,SystemTime};
use chrono::prelude::*;
use crate::io::{AppState,Event};

pub enum TimerState {
    Paused, 
    Running,
}

pub struct Clock {
    pub seconds :i32,
    pub minutes :i32,
    pub hours   :i32
}

impl From<i32> for Clock {
    fn from(seconds: i32) -> Self {
        let mut minutes = seconds / 60;
        let     hours   = minutes / 60;

        minutes = minutes % 60;
        let     seconds = seconds % 60;

        Clock{seconds, minutes, hours}
    }
}

pub struct Times {
    pub total  :i32,
    pub split  :i32,
    pub day    :i32,
    pub week   :i32,
    pub month  :i32,
}

pub enum TimeView {
    Total,
    Split,
    Day,
    Week,
    Month,
}

pub struct Timer {
    pub view    :TimeView,
    pub state   :TimerState,
    pub name    :String,
    pub now         :SystemTime,
    pub interval    :Duration,
    pub start       :DateTime<Local>,

    pub times: Times,
}

impl Timer {
    pub fn get_view(&self) -> String {
        use TimeView::*;
        match self.view {
            Split => String::from("Split"),
            Day   => String::from("Day"),
            Week  => String::from("Week"),
            Month => String::from("Month"),
            Total => String::from("Total"),
        }
    }
}

impl Timer {
    pub fn get_clock(&self) -> Clock {
        use TimeView::*;
        match self.view {
            Total => Clock::from(self.times.total + self.times.split),

            Month => Clock::from(self.times.month + self.times.split),
            Week  => Clock::from(self.times.week  + self.times.split),
            Day   => Clock::from(self.times.day   + self.times.split),

            Split => Clock::from(self.times.split),
        }
    }
}


pub fn timer(tx: &Sender<Event>, state: &mut AppState) {
    // check if 1 second has elapsed
    if state.timer.now.elapsed().unwrap() >= state.timer.interval {
        if let TimerState::Running = state.timer.state {
            state.timer.times.split += 1; 

            tx.send(Event::Tick(state.timer.get_clock())).unwrap(); 
        }
        state.timer.now = state.timer.now + state.timer.interval; 
    }
}

