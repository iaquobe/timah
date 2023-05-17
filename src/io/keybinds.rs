use ncurses::*;

pub enum ActionNormal {
    Pause,
    Quit,
    OpenList,
    Rename,
    SwitchView,
    SwitchTimerAccumulate,
}

pub enum ActionName {
    Confirm,
    Cancel,
    Delete,
    Type(char),
}

pub enum ActionList {
    Up,
    Down,

    Confirm,
    Cancel,
}

pub enum Action {
    Normal(ActionNormal),
    List(ActionList),
    Name(ActionName),

    None,
}

pub enum Mode {
    Normal,
    List, 
    Name,
}


pub fn get_action(mode:&Mode, c:i32) -> Action {
    const ESC:i32 = 27;
    const ENTER:i32 = '\n' as i32;
    const SPACE:i32 = ' ' as i32; 
    const Q:i32 = 'q' as i32; 
    const N:i32 = 'n' as i32; 
    const O:i32 = 'o' as i32;
    const J:i32 = 'j' as i32;
    const K:i32 = 'k' as i32;
    const T:i32 = 't' as i32;
    const A:i32 = 'a' as i32;

    match mode {
        Mode::Normal => { match c {
            Q       => Action::Normal(ActionNormal::Quit),
            O       => Action::Normal(ActionNormal::OpenList),
            N       => Action::Normal(ActionNormal::Rename),
            T       => Action::Normal(ActionNormal::SwitchView),
            A       => Action::Normal(ActionNormal::SwitchTimerAccumulate),
            SPACE   => Action::Normal(ActionNormal::Pause),
            _       => Action::None,
        }},
        Mode::Name   => { match c {
            KEY_BACKSPACE => Action::Name(ActionName::Delete),
            ENTER         => Action::Name(ActionName::Confirm),
            ESC           => Action::Name(ActionName::Cancel),
            _             => match char::from_u32(c as u32) {
                Some(ch)    => Action::Name(ActionName::Type(ch)),
                None        => Action::None,
        }}},
        Mode::List   => { match c {
            Q | ESC => Action::List(ActionList::Cancel),
            J       => Action::List(ActionList::Down),
            K       => Action::List(ActionList::Up),
            ENTER   => Action::List(ActionList::Confirm),
            _       => Action::None,
        }},
    }
}
