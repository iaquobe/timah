use std::sync::mpsc::Sender;
use super::*; 

/// state of the app
pub struct AppState {
    // main state
    pub sender    :Sender<Event>,
    pub mode      :Mode,
    pub timer     :Timer,
    pub pomodore  :Pomodore,

    // list data
    pub timers    :Vec<String>,
    pub selection :usize, 

    // other data 
    pub path      :String,
    pub prev_name :String,
}
