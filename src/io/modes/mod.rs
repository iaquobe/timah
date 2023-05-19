use std::sync::mpsc::Sender;
use super::files::read_all_timers;
use super::keybinds::*;
use super::timer::*;
use super::files;
use chrono::prelude::*;


mod list;
mod normal;
mod rename;

/// weither the program should stop or not 
pub enum Control {
    Continue,
    Break,
}

/// state of the app
pub struct AppState {
    // main state
    pub sender    :Sender<Event>,
    pub mode      :Mode,
    pub timer     :Timer,

    // list data
    pub timers    :Vec<String>,
    pub selection :usize, 

    // other data 
    pub path      :String,
    pub prev_name :String,
}

/// all events that can be sent to the ui
pub enum Event {
    Init{timer:String, timeframe:String, legend:String, clock:Clock},
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
}

/// delegates to action handler of the current mode
pub fn handle_action(state:&mut AppState, action:Action) -> Control {
    match action {
        Action::Normal(action)    => normal::normal_mode(state, action),
        Action::Name(action)      => rename::rename_mode(state, action),
        Action::List(action)      => list::list_mode(state, action),
        Action::None              => Control::Continue,
    }
}




