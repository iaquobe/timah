use std::sync::mpsc::Sender;
use super::*;

pub enum TimerAccumulate {
    Total, 
    Timer,
}

pub enum TimeFrame {
    Total,
    Split,
    Day,
    Week,
    Month,
}

pub enum TimerState {
    Paused, 
    Running,
}


pub struct Timer {
    // information relevant for ui
    pub view    :TimeFrame,
    pub state   :TimerState,
    pub name    :String,

    // accumulation of realtime and logged time 
    pub mode : TimerAccumulate,
    pub times: Times,
    pub total: Times,

    // realtime tracking
    pub now         :SystemTime,
    pub interval    :Duration,
    pub start       :DateTime<Local>,
}

impl Timer {
    /// return the name of the selected mode (split, day, week, month, total) 
    pub fn get_view(&self) -> String {
        use TimeFrame::*;
        match self.view {
            Split => String::from("Split"),
            Day   => String::from("Day"),
            Week  => String::from("Week"),
            Month => String::from("Month"),
            Total => String::from("Total"),
        }
    }

    /// get name of the timer, and append "(All)" if accumulation is on
    pub fn get_name(&self) -> String {
        self.name.clone().push_str("(All)");
        match self.mode {
            TimerAccumulate::Timer => self.name.clone(),
            TimerAccumulate::Total => format!("{}(All)", self.name),
        }
    }

    /// return time as Clock (hours, minutes, seconds)
    /// depending on the mode (split, day, week, ...) a different time is computed
    pub fn get_clock(&self) -> Clock {
        use TimeFrame::*;

        let times = match self.mode {
            TimerAccumulate::Timer => &self.times,
            TimerAccumulate::Total => &self.total,
        };

        match self.view {
            Total => Clock::from(times.total + self.times.split),

            Month => Clock::from(times.month + self.times.split),
            Week  => Clock::from(times.week  + self.times.split),
            Day   => Clock::from(times.day   + self.times.split),

            Split => Clock::from(self.times.split),
        }
    }

    /// increases timer by seconds and send information about changed values to ui
    pub fn tick(&mut self, seconds: i32, sender: &Sender<Event>) {
        self.times.split += seconds;
        sender.send(Event::Tick(self.get_clock())).unwrap(); 
    }
}


