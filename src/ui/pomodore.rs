use super::*;

pub fn print(ws: &mut WindowState){
    let timer_str = format!("{}: {:02}:{:02}", ws.pomodore_mode, ws.pomodore_clock.minutes, ws.pomodore_clock.seconds);

    mvwprintw(ws.pomodore_win, 0, 0, &timer_str);
    wrefresh(ws.pomodore_win);
}
