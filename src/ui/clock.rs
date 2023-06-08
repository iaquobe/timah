use super::*;
use shapes::SHAPES;

pub fn print(ws: &mut WindowState){
    let (timer_str,format) = 
        if ws.clock.hours > 99 {
            (format!("{:04}:{:02}", ws.clock.hours, ws.clock.minutes), ClockFormat::HHHHMM) 
        } else {
            (format!("{:02}:{:02}:{:02}", ws.clock.hours, ws.clock.minutes, ws.clock.seconds), ClockFormat::HHHHMM) 
        };

    let mut start = 0; 

    if ws.timer_format != format {
        wclear(ws.timer_win);
        ws.timer_format = format; 
    }

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
