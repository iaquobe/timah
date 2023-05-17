use super::*;

pub fn print(ws: &WindowState){
    wclear(ws.title_win);
    let offset = ws.width - (ws.view.len() as i32) - 1;
    mvwprintw(ws.title_win, 0, offset, &ws.view);
    mvwprintw(ws.title_win, 0, 0     , &ws.title);
    wrefresh(ws.title_win);
}

