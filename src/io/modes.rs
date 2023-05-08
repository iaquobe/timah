use std::sync::mpsc::Sender;
use crate::io::keybinds::*;
use crate::io::timer::*;
use crate::io::files;
use chrono::prelude::*;

/// weither the program should stop or not 
pub enum Control {
    Continue,
    Break,
}

/// state of the app
pub struct AppState {
    pub mode      :Mode,
    pub timer     :Timer,
    pub timers    :Vec<String>,
    pub selection :usize, 
    pub path      :String,
    pub prev_name :String,
}

/// all events that can be sent to the ui
pub enum Event {
    Init(String,Times),
    Quit,
    Resize,
    Tick(Times),

    NameOpen(String),
    NameClose,
    NameTick(String),

    TimersOpen(Vec<String>),
    TimersSelect(usize),
    TimersClose,
}

/// delegates to action handler of the current mode
pub fn handle_action(tx: &Sender<Event>, state:&mut AppState, action:Action) -> Control {
    match action {
        Action::Normal(action)    => normal_mode(tx, state, action),
        Action::Name(action)      => name_mode(tx, state, action),
        Action::List(action)      => list_mode(tx, state, action),
        Action::None              => Control::Continue,
    }
}


/// handles possible actions in normal mode
fn normal_mode(tx: &Sender<Event>, state:&mut AppState, action:ActionNormal) -> Control {
    use ActionNormal::*;
    match action {
        Quit        => {
            // change state
            tx.send(Event::Quit).unwrap();
            if let TimerState::Running = state.timer.state {
                let end = Local::now();
                files::write_timer(&state.path, &state.timer.name, &state.timer.start, &end);
            }
            //send to ui
            return Control::Break;
        },
        Pause       => { 
            // change state
            state.timer.state = match state.timer.state {
                TimerState::Running => {
                    let end = Local::now();
                    files::write_timer(&state.path, &state.timer.name, &state.timer.start, &end);
                    TimerState::Paused
                }, 
                TimerState::Paused  => {
                    state.timer.start = Local::now();
                    TimerState::Running
                },}
        },
        Rename      => {
            // change state
            state.mode       = Mode::Name;
            state.timer.name = String::from("");
            // send to ui
            tx.send(Event::NameOpen(state.timer.name.clone())).unwrap();
        },
        OpenList    => {
            // change state
            state.mode      = Mode::List; 
            state.selection = 0;
            state.timers = files::read_timers(&state.path);
            // send to ui
            tx.send(Event::TimersOpen(state.timers.clone())).unwrap();
        },
    }

    Control::Continue
}


/// handles possible actions in naming mode
fn name_mode(tx: &Sender<Event>, state:&mut AppState, action:ActionName) -> Control {
    use ActionName::*;
    match action {
        Cancel  => {
            // change state 
            state.timer.name = state.prev_name.clone();
            state.mode       = Mode::Normal;
            // send to ui
            tx.send(Event::NameClose).unwrap();
        },
        Confirm => {
            // change state
            state.mode = Mode::Normal;
            state.timer.seconds += files::read_timer(&state.path, &state.timer.name);
            //send to ui
            tx.send(Event::Tick(Times::from(state.timer.seconds))).unwrap();
            tx.send(Event::NameClose).unwrap();
        },
        Delete  => {
            // change state
            state.timer.name.pop();
            // send to ui
            tx.send(Event::NameTick(state.timer.name.clone())).unwrap();
        },
        Type(c) => {
            // change state
            state.timer.name.push(c);
            // send to ui
            tx.send(Event::NameTick(state.timer.name.clone())).unwrap();
        },
    }
    Control::Continue
}


/// handles possible actions in list mode
fn list_mode(tx: &Sender<Event>, state:&mut AppState, action:ActionList) -> Control {
    use ActionList::*;
    match action {
        Cancel  => {
            // change state
            state.mode = Mode::Normal;
            // send to ui
            tx.send(Event::TimersClose).unwrap();
        },
        Up      => {
            // change state
            if state.selection > 0 { state.selection -= 1; }
            // send to ui
            tx.send(Event::TimersSelect(state.selection)).unwrap();
        },
        Down    => {
            // change state
            if state.selection + 1 < state.timers.len() { state.selection += 1; }
            // send to ui
            tx.send(Event::TimersSelect(state.selection)).unwrap();
        },
        Confirm => {
            // change state
            state.mode          = Mode::Normal;
            state.timer.name    = state.timers.get(state.selection).unwrap_or(&String::from("")).clone();
            state.timer.seconds = files::read_timer(&state.path, &state.timer.name);
            // send to ui
            tx.send(Event::TimersSelect(0)).unwrap();
            tx.send(Event::TimersClose).unwrap();
            tx.send(Event::NameTick(state.timer.name.clone())).unwrap();
            tx.send(Event::NameClose).unwrap();
            tx.send(Event::Tick(Times::from(state.timer.seconds))).unwrap();
        },
    }
    Control::Continue
}
