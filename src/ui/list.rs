use super::*;

pub fn print(ws:&mut WindowState){
    wclear(ws.files_win);
    box_(ws.files_win, 0, 0);

    // create iter for the files that are shown
    let (selected, iter) =
        // on begin of the list, show highlight depending on the index
        if ws.selected < FILES_SHOWN {
            (ws.selected + 1, 
             ws.files.iter()
                 .skip(0)
                 .take(FILES_SHOWN))
        }
        // if further down the list, then highlight will be the last timer
        else {
            (FILES_SHOWN, 
             ws.files.iter()
                 .skip(ws.selected - FILES_SHOWN + 1)
                 .take(FILES_SHOWN))
        };

    // print the files, and highligth the selected file
    let mut row = 1; 
    for timer in iter {
        if selected == row {
            wattron(ws.files_win, A_REVERSE());
        }

        mvwprintw(ws.files_win, row as i32, 1, &timer);

        if selected == row {
            wattroff(ws.files_win, A_REVERSE());
        }
        row += 1; 
    }

    wrefresh(ws.files_win);
}

