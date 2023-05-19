use super::*;

pub fn print(ws: &WindowState){
    wclear(ws.legend_win);
    if ws.legend_show {
        mvwprintw(ws.legend_win, 0, 0, &ws.legend);
    }
    wrefresh(ws.legend_win);
}
