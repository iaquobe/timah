use ncurses::*;
use std::sync::mpsc;
use crate::shapes::SHAPES; 
use crate::io::*;

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

struct WindowState {
    // general info about window
    rows    :i32,
    cols    :i32,
    width   :i32, 
    height  :i32,

    // data about title window
    title_win   :*mut i8, 
    title       :String,

    // data about timer
    timer_win   :*mut i8, 
    times       :Times,

    // data about file(when opening other timers)
    files_show  :bool,
    files_win   :*mut i8, 
    files       :Vec<String>,
    selected    :usize,
}

pub fn ui_thread(rx: mpsc::Receiver<Event>){
    // init state
    let mut ws = match rx.recv().unwrap() {
        Event::Init(name, times) => init_state(name, times),
        _                  => {panic!("expected init event")},
    };

    //initial print
    print_all(&mut ws);


    for event in rx {
        match event {
            Event::Resize                 => { resize_window(&mut ws)},
            Event::Tick(times)            => { ws.times = times; print_time(&mut ws)},
            Event::Quit                   => break,
            Event::NameOpen(name) 
                | Event::NameTick(name)   => { curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE); ws.title = name; print_title(&mut ws)},
            Event::NameClose              => { curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);},
            Event::TimersOpen(timers)     => { ws.files = timers; print_timers(&mut ws)},
            Event::TimersClose            => { wclear(ws.files_win); wrefresh(ws.files_win);},
            Event::Init(_,_)            => {},
            Event::TimersSelect(selected) => { ws.selected = selected; print_timers(&mut ws)},
        }
    }
}

fn init_state(name:String, times:Times) -> WindowState{
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
        times,

        title_win,
        title,
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
    print_time(ws);
    print_title(ws);
    
    if ws.files_show { print_timers(ws); }
}


fn print_time(ws: &WindowState){
    let timer_str = format!("{:02}:{:02}:{:02}", ws.times.hours, ws.times.minutes, ws.times.seconds);
    let mut start = 0; 

    for c in timer_str.chars() {
        let (symbol,inc) = 
            if c >= '0' && c <= '9'{
                (SHAPES[((c as u8) - b'0') as usize], CHAR_INC)
            } else { 
                (SHAPES[10], COLON_INC) 
            }; 

        let mut column = 0;
        for line in symbol.lines() {
            mvwprintw(ws.timer_win, column, start, line);
            column += 1; 
        }
        start += inc; 
    }
    wrefresh(ws.timer_win);
}


fn print_title(ws: &WindowState){
    wclear(ws.title_win);
    mvwprintw(ws.title_win, 0, 0, ws.title.as_ref());
    wrefresh(ws.title_win);
}


fn print_timers(ws:&mut WindowState){
    wclear(ws.files_win);
    box_(ws.files_win, 0, 0);

    let (from, to, hi) = if  FILES_SHOWN >= ws.files.len() {
        (0, ws.files.len(), ws.selected)
    } else if ws.selected + FILES_SHOWN > ws.files.len() {
        let len = ws.files.len(); 

        (len - FILES_SHOWN, len, FILES_SHOWN + 1 + ws.selected - len)
    } else {
        (ws.selected, ws.selected + FILES_SHOWN, 1)
    };

    let mut row = 1; 
    for timer in &ws.files[from..to] {
        if hi == row {
            wattron(ws.files_win, A_REVERSE());
        }

        mvwprintw(ws.files_win, row as i32, 1, &timer);

        if hi == row {
            wattroff(ws.files_win, A_REVERSE());
        }
        row += 1; 
    }
    wrefresh(ws.files_win);
}

