use std::ops::Add;
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

impl Add for Times {
    type Output = Self; 

    fn add(self, rhs: Self) -> Self {
        Self { 
            split:  self.split  + rhs.split,
            day:    self.day    + rhs.day,
            week:   self.week   + rhs.week,
            month:  self.month  + rhs.month,
            total:  self.total  + rhs.total,
        }
    }
}

impl Default for Times {
    fn default() -> Self {
        Self { total: 0, split: 0, day: 0, week: 0, month: 0 }
    }
}


pub enum TimeMode {
    Total, 
    Timer,
}

pub enum TimeView {
    Total,
    Split,
    Day,
    Week,
    Month,
}

pub struct Timer {
    // information relevant for ui
    pub view    :TimeView,
    pub state   :TimerState,
    pub name    :String,

    // accumulation of realtime and logged time 
    pub mode : TimeMode,
    pub times: Times,
    pub total: Times,

    // realtime tracking
    pub now         :SystemTime,
    pub interval    :Duration,
    pub start       :DateTime<Local>,
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

    pub fn get_name(&self) -> String {
        self.name.clone().push_str("(All)");
        match self.mode {
            TimeMode::Timer => self.name.clone(),
            TimeMode::Total => format!("{}(All)", self.name),
        }
    }

    pub fn get_clock(&self) -> Clock {
        use TimeView::*;

        let times = match self.mode {
            TimeMode::Timer => &self.times,
            TimeMode::Total => &self.total,
        };

        match self.view {
            Total => Clock::from(times.total + self.times.split),

            Month => Clock::from(times.month + self.times.split),
            Week  => Clock::from(times.week  + self.times.split),
            Day   => Clock::from(times.day   + self.times.split),

            Split => Clock::from(self.times.split),
        }
    }
}


pub fn timer(tx: &Sender<Event>, state: &mut AppState) {
    // check if 1 second has elapsed
    if let Ok(elapsed) = state.timer.now.elapsed() {
        let elapsed_seconds = elapsed.as_secs(); 
        if let TimerState::Running = state.timer.state {
            state.timer.times.split += elapsed_seconds as i32;
            tx.send(Event::Tick(state.timer.get_clock())).unwrap(); 
        }
        state.timer.now = state.timer.now + Duration::from_secs(elapsed_seconds);
    }
}

