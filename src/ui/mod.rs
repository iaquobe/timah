use ncurses::*;
use std::sync::mpsc;
use crate::io::*;

mod list;
mod title;
mod clock;
mod shapes;

pub struct WindowState {
    // general info about window
    pub rows    :i32,
    pub cols    :i32,
    pub width   :i32, 
    pub height  :i32,

    // data about title window
    pub title_win   :*mut i8, 
    pub title       :String,
    pub view        :String,

    // data about timer
    pub timer_win   :*mut i8, 
    pub clock       :Clock,

    // data about file(when opening other timers)
    pub files_show  :bool,
    pub files_win   :*mut i8, 
    pub files       :Vec<String>,
    pub selected    :usize,
}


const FILES_SHOWN:usize = 3; 
const CHAR_HEIGHT:i32 = 5; 
const CHAR_WIDTH:i32 = 5; 
const CHAR_MARGING:i32 = 1;
const CHAR_NUM:i32 = 6;
const COLON_WIDTH: i32 = 1;
const COLON_MARGIN: i32 = 1;
const COLON_NUM: i32 = 2;
const CHAR_INC:i32 = CHAR_WIDTH + CHAR_MARGING;
const COLON_INC: i32 = COLON_WIDTH + COLON_MARGIN;
const CLOCK_WIN_WIDTH:i32 = CHAR_NUM * CHAR_INC + COLON_NUM * COLON_INC; 

pub fn ui_thread(rx: mpsc::Receiver<Event>){
    // init state
    let mut ws = match rx.recv().unwrap() {
        Event::Init(name, clock, times) => init_state(name, clock, times),
        _                  => {panic!("expected init event")},
    };

    //initial print
    print_all(&mut ws);


    for event in rx {
        match event {
            Event::Resize                 => { resize_window(&mut ws)},
            Event::Tick(clock)            => { ws.clock = clock; clock::print(&mut ws)},
            Event::Quit                   => break,
            Event::NameView(view)         => { ws.view = view; title::print(&mut ws)},
            Event::NameOpen(name) 
                | Event::NameTick(name)   => { curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE); ws.title = name; title::print(&mut ws)},
            Event::NameClose              => { curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);},
            Event::TimersOpen(timers)     => { ws.files = timers; list::print(&mut ws)},
            Event::TimersClose            => { wclear(ws.files_win); wrefresh(ws.files_win);},
            Event::Init(_,_,_)            => {},
            Event::TimersSelect(selected) => { ws.selected = selected; list::print(&mut ws)},
        }
    }
}

fn init_state(name:String, view: String, clock:Clock) -> WindowState{
    // Get the screen size
    let mut rows = 0;
    let mut cols = 0;
    getmaxyx(stdscr(), &mut rows, &mut cols);

    //let legend_win = newwin(1          , cols , rows - 1       , 0) ; 

    // Get windows
    let width  = CLOCK_WIN_WIDTH;
    let height = CHAR_HEIGHT;
    let y  = (rows - height) / 2;
    let x  = (cols - width) / 2;
    let timer_win  = newwin(height, width, y         , x);
    let title_win  = newwin(1     , width, y - 1     , x); 
    let files_win  = newwin(height, width, y + height, x);

    let title      = name.clone();

    WindowState {
        rows,
        cols,
        width,
        height,

        timer_win,
        clock,

        title_win,
        title,
        view,
        files: vec![] ,

        files_show: false,
        files_win,
        selected: 0,
    }
}


fn resize_window(ws:&mut WindowState) {
    mvwin(stdscr(), 0, 0);
    getmaxyx(stdscr(), &mut ws.rows, &mut ws.cols);
    let y = (ws.rows - CHAR_HEIGHT) / 2;
    let x = (ws.cols - ws.width) / 2;

    mvwin(ws.timer_win , y            , x);
    mvwin(ws.title_win , y - 1        , x);
    mvwin(ws.files_win , y + ws.height, x);

    refresh();
    print_all(ws);
}


fn print_all(ws:&mut WindowState){
    clock::print(ws);
    title::print(ws);

    if ws.files_show { list::print(ws); }
}
