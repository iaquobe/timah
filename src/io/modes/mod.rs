use super::files::read_all_timers;
use super::keybinds::*;
use super::files;
use chrono::prelude::*;

pub use super::timer::*;

mod list;
mod normal;
mod rename;
mod event;
mod app_state;
pub use event::*;
pub use app_state::*;

/// weither the program should stop or not 
pub enum Control {
    Continue,
    Break,
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




