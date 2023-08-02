use super::*; 

/// all events that can be sent to the ui
pub enum Event {
    Init{timer:String, timeframe:String, legend:String, pomodore:String, clock:Clock, pomodore_clock:Clock},
    Quit,
    Resize,
    Tick(Clock),

    NameOpen(String),
    NameView(String),
    NameClose,
    NameTick(String),

    TimersOpen(Vec<String>),
    TimersSelect(usize),
    TimersClose,

    LegendUpdate(String),
    LegendToggle,

    PomodoreTick(Clock),
    PomodoreName(String),
    PomodoreToggle,
}
