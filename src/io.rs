use std::io::{prelude::*,BufRead};
use std::time::{Duration,Instant};
use std::sync::mpsc::{self, Sender};
use std::fs::{self, OpenOptions}; 
use chrono::prelude::*;
use std::path::Path;
use ncurses::*;

use shellexpand;

const CACHE_DIR:&str = "~/.cache/taimah/";

pub const QUIT:i32 = 'q' as i32; 
pub const PAUSE:i32 = ' ' as i32; 
pub const NAME:i32 = 'n' as i32; 
pub const ENTER:i32 = '\n' as i32;
pub const OPEN:i32 = 'o' as i32;
pub const ESC:i32 = 27;
pub const DOWN:i32 = 'j' as i32;
pub const UP:i32 = 'k' as i32;
pub const TOGGLE_LEGEND:i32 = '\t' as i32;

enum Control {
    Continue,
    Break,
}

// events that are shared on the 
pub enum Event {
    Init(String,Times,String),
    Quit,
    Resize,
    Tick(Times),

    NameOpen(String),
    NameClose,
    NameTick(String),

    TimersOpen(Vec<String>),
    TimersSelect(usize),
    TimersClose,

    LegendToggle,
    LegendText(String),
}

enum TimerState {
    Paused, 
    Running,
}

pub struct Times {
    pub seconds :i32,
    pub minutes :i32,
    pub hours   :i32
}

impl From<i32> for Times {
    fn from(seconds: i32) -> Self {
        let mut minutes = seconds / 60;
        let     hours   = minutes / 60;

                minutes = minutes % 60;
        let     seconds = seconds % 60;

        Times{seconds, minutes, hours}
    }
}


struct Timer {
    state   :TimerState,
    name    :String,
    seconds :i32,
    now         :Instant,
    interval    :Duration,
    start       :DateTime<Local>,
}

enum Mode {
    Normal,
    Name,
    List,
}

struct AppState {
    mode      :Mode,
    timer     :Timer,
    timers    :Vec<String>,
    selection :usize, 
    path      :String,

    prev_name :String,
}

pub fn io_thread(tx: mpsc::Sender<Event>){
    let mut state = AppState{
        mode      : Mode::Normal,

        prev_name : String::from(""),
        selection : 0,
        timers    : vec![],
        path      : String::from(shellexpand::tilde(CACHE_DIR)),

        timer     : Timer{
            state    : TimerState::Paused,
            name     : String::from(""),
            now      : Instant::now(),
            interval : Duration::from_secs(1),
            start    : Local::now(),
            seconds  : 0,
        },
    };

    tx.send(Event::Init(state.timer.name.clone(), Times::from(state.timer.seconds), get_legend(&state.mode)))
        .unwrap();

    loop {
        let c = getch(); 

        // mode independent inputs
        if c == KEY_RESIZE {
            tx.send(Event::Resize).unwrap();
        }

        // mode dependent inputs
        if let Control::Break = match state.mode {
            Mode::Normal => normal_mode(&tx, &mut state, c),
            Mode::Name   => name_mode(&tx, &mut state, c),
            Mode::List   => list_mode(&tx, &mut state, c),
        } { break; }

        timer(&tx, &mut state);

        napms(10);
    }
}


fn timer(tx: &Sender<Event>, state: &mut AppState) {
    // check if 1 second has elapsed
    if state.timer.now.elapsed() >= state.timer.interval {
        if let TimerState::Running = state.timer.state {
            state.timer.seconds += 1; 

            tx.send(Event::Tick(Times::from(state.timer.seconds))).unwrap(); 
        }
        state.timer.now = state.timer.now + state.timer.interval; 
    }
}


fn normal_mode(tx: &Sender<Event>, state:&mut AppState, c:i32) -> Control {
    match c {
        QUIT       => {
            tx.send(Event::Quit).unwrap();
            return Control::Break;
        },
        PAUSE      => { 
            state.timer.state = match state.timer.state {
                TimerState::Running => {
                    write_timer(state);
                    TimerState::Paused
                }, 
                TimerState::Paused  => {
                    state.timer.start = Local::now();
                    TimerState::Running
                }, 
            }
        },
        NAME => {
            state.mode       = Mode::Name;
            state.timer.name = String::from("");

            tx.send(Event::LegendText(get_legend(&state.mode))).unwrap();
            tx.send(Event::NameOpen(state.timer.name.clone())).unwrap();
        },
        OPEN => {
            state.mode      = Mode::List; 
            state.selection = 0;

            state.timers = read_timers(CACHE_DIR);

            tx.send(Event::LegendText(get_legend(&state.mode))).unwrap();
            tx.send(Event::TimersOpen(state.timers.clone())).unwrap();
        },
        TOGGLE_LEGEND => {
            tx.send(Event::LegendToggle).unwrap();
        },
        _ => {},
    }

    Control::Continue
}


fn name_mode(tx: &Sender<Event>, state:&mut AppState, c:i32) -> Control {
    match c {
        ESC               => {
            state.timer.name = state.prev_name.clone();
            state.mode       = Mode::Normal;

            tx.send(Event::LegendText(get_legend(&state.mode))).unwrap();
            tx.send(Event::NameClose).unwrap();
        },
        KEY_ENTER | ENTER => {
            state.mode = Mode::Normal;

            tx.send(Event::LegendText(get_legend(&state.mode))).unwrap();
            tx.send(Event::NameClose).unwrap();
        },
        KEY_BACKSPACE     => {
            state.timer.name.pop();

            tx.send(Event::NameTick(state.timer.name.clone())).unwrap();
        },
        _                 => {
            // if number represents a char
            if let Some(ch) = char::from_u32(c as u32) {
                state.timer.name.push(ch);
                tx.send(Event::NameTick(state.timer.name.clone())).unwrap();
            }
        },
    }

    Control::Continue
}


fn list_mode(tx: &Sender<Event>, state:&mut AppState, c:i32) -> Control {
    match c {
        ESC | QUIT => {
            state.mode = Mode::Normal;

            tx.send(Event::LegendText(get_legend(&state.mode))).unwrap();
            tx.send(Event::TimersClose).unwrap();
        },
        UP => {
            if state.selection > 0 {
                state.selection -= 1;
            }
            tx.send(Event::TimersSelect(state.selection)).unwrap();
        },
        DOWN => {
            if state.selection < state.timers.len() - 1 {
                state.selection += 1;
            }
            tx.send(Event::TimersSelect(state.selection)).unwrap();

        },
        ENTER => {
            state.mode = Mode::Normal;
            state.timer.name    = state.timers[state.selection].clone();
            state.timer.seconds = read_timer(&state.path, &state.timer.name);

            tx.send(Event::LegendText(get_legend(&state.mode))).unwrap();
            tx.send(Event::TimersSelect(0)).unwrap();
            tx.send(Event::TimersClose).unwrap();
            tx.send(Event::NameTick(state.timers[state.selection].clone())).unwrap();
            tx.send(Event::NameClose).unwrap();
            tx.send(Event::Tick(Times::from(state.timer.seconds))).unwrap();
            tx.send(Event::Tick(Times::from(state.timer.seconds))).unwrap();
        },
        _  => {},
    }
    Control::Continue
}


fn write_timer(state:&AppState) {
    let path = Path::new(&state.path).join(&state.timer.name);
    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        {
            let end  = Local::now();
            writeln!(file, "{}---{}", state.timer.start, end).unwrap();
        }
}


fn read_timer(dir:&str, name:&str) -> i32 {
    // read file and add up all time splits 
    let path = Path::new(dir).join(name);
    fs::read(path).unwrap()
        .lines()
        .fold(0, |sec, line| {
            let line = line.unwrap();
            let mut iter = line.split("---");
            // try to read dates, otherwise its a reset char
            match (iter.next().unwrap().parse::<DateTime<Local>>(), iter.next().unwrap().parse::<DateTime<Local>>()) {
                (Ok(start), Ok(end)) => {
                    sec + (end - start).num_seconds()
                }, 
                _ => 0,
            }
        }) as i32
}


fn read_timers(path:&str) -> Vec<String> {
    fs::read_dir(shellexpand::tilde(path).as_ref()).unwrap()
        .map(|path| {
            String::from(path.unwrap()
                         .path()
                         .file_name()
                         .unwrap_or_default()
                         .to_string_lossy())})
        .collect()
}

fn get_legend(mode:&Mode) -> String{
    match mode {
        Mode::Normal => { format!("quit({}) pause/play({}) rename({}) open({})", QUIT, PAUSE, NAME, OPEN) },
        Mode::Name   => { format!("quit(esc) confirm(enter) cancel(esc)") },
        Mode::List   => { format!("up/down({}/{}) confirm(enter) cancel(esc)", UP, DOWN) },
    }

}
