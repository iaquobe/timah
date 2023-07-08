use std::sync::mpsc::Sender;
use super::*;

pub enum PomodoreMode {
    ShortBreak, 
    Break, 
    Work,
}

pub struct Pomodore {
    pub mode     :PomodoreMode,
    pub pomodores:u32,

    pub time: i32,
}

impl Default for Pomodore {
    fn default() -> Self {
        Self { mode : PomodoreMode::Work, pomodores: 0, time: 0 }
    }
}

impl Pomodore {
    /// get name of pomodore mode
    pub fn get_mode(&self) -> String {
        match self.mode {
            PomodoreMode::Work       => String::from("Work"),
            PomodoreMode::Break      => String::from("Break"),
            PomodoreMode::ShortBreak => String::from("ShortBreak"),
        }
    }

    /// return remaining time of pomodore mode
    pub fn get_clock(&self) -> Clock {
        let max = match self.mode {
            PomodoreMode::Work       => 25 * 60,
            PomodoreMode::Break      =>  5 * 60,
            PomodoreMode::ShortBreak => 25 * 60,
        };

        Clock::from(max - self.time)
    }

    /// increases timer by seconds and send information about changed values to ui
    pub fn tick(&mut self, seconds: i32, sender: &Sender<Event>) {
        self.time += seconds;

        let max = match self.mode {
            PomodoreMode::Work       => 25 * 60,
            PomodoreMode::Break      =>  5 * 60,
            PomodoreMode::ShortBreak => 25 * 60,
        };

        if self.time >= max {
            self.time -= max;

            self.mode = match self.mode {
                PomodoreMode::Break      => PomodoreMode::Work,
                PomodoreMode::ShortBreak => PomodoreMode::Work,
                PomodoreMode::Work       => {
                    self.pomodores = (self.pomodores + 1) % 4; 
                    if self.pomodores == 0 {
                        PomodoreMode::Break
                    }
                    else {
                        PomodoreMode::ShortBreak
                    }
                },
            };

            sender.send(Event::PomodoreName(self.get_mode())).unwrap();
        }
        sender.send(Event::PomodoreTick(self.get_clock())).unwrap();
    }
}
