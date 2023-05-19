use ncurses::*;

pub enum ActionNormal {
    Pause,
    Quit,
    OpenList,
    Rename,
    TimeFrameNext,
    TimeFramePrevious,
    SwitchTimerAccumulate,

    LegendToggle,
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

    LegendToggle,
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
    const ESC  :i32 = 27;
    const ENTER:i32 = '\n' as i32;
    const TAB  :i32 = '\t' as i32;
    const SPACE:i32 = ' ' as i32; 
    const Q:i32 = 'q' as i32; 
    const N:i32 = 'n' as i32; 
    const O:i32 = 'o' as i32;
    const J:i32 = 'j' as i32;
    const K:i32 = 'k' as i32;
    const A:i32 = 'a' as i32;
    const T:i32 = 't' as i32;
    const T_U:i32 = 'T' as i32;

    match mode {
        Mode::Normal => { match c {
            Q       => Action::Normal(ActionNormal::Quit),
            O       => Action::Normal(ActionNormal::OpenList),
            N       => Action::Normal(ActionNormal::Rename),
            T       => Action::Normal(ActionNormal::TimeFrameNext),
            T_U     => Action::Normal(ActionNormal::TimeFramePrevious),
            A       => Action::Normal(ActionNormal::SwitchTimerAccumulate),
            SPACE   => Action::Normal(ActionNormal::Pause),
            TAB     => Action::Normal(ActionNormal::LegendToggle),
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
            TAB     => Action::List(ActionList::LegendToggle),
            _       => Action::None,
        }},
    }
}

pub fn get_legend(mode:&Mode) -> String {
    match mode {
        Mode::Normal => String::from("quit(q), open timers(o), rename(n), switch timeframe(t/T), toggle accumulate(a), pause/start(space), toggle help(tab)"),
        Mode::Name   => String::from("confirm(enter), cancel(escape)"),
        Mode::List   => String::from("cancel(escape/q), confirm(enter), down/up(j/k), toggle help(tab)"),
    }
}
