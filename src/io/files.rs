use std::path::Path;
use std::fs::{self,OpenOptions};
use std::io::{Write,BufRead};
use chrono::{Local,DateTime, Datelike};
use crate::io::timer::Times;

const ALL_TIMERS:&str = "All";

pub fn write_timer(dir:&str, name:&str, start:&DateTime<Local>, end:&DateTime<Local>) {
    let path = Path::new(&dir).join(&name);
    if let Ok(mut file) = OpenOptions::new()
        .write(true)
            .append(true)
            .create(true)
            .open(path)
            {
                writeln!(file, "{}---{}", start, end).unwrap();
            }
}

fn read_timer(path:&Path) -> Times {
    let now = Local::now(); 
    match fs::read(path) {
        Ok(file) => {
            file.lines()
                .fold(Times{ day: 0, week: 0, month: 0, total: 0, split: 0, }, |mut times, line| {
                    let line = line.unwrap();
                    let mut iter = line.split("---");
                    // try to read dates, otherwise its a reset char
                    match (iter.next().unwrap().parse::<DateTime<Local>>(), iter.next().unwrap().parse::<DateTime<Local>>()) {
                        (Ok(start), Ok(end)) => {
                            let split = (end - start).num_seconds() as i32;

                            times.total += split;
                            if start.year() == now.year() {
                                if start.month() == now.month() {
                                    times.month += split;
                                }
                                if start.iso_week() == now.iso_week() {
                                    times.week += split;
                                }
                                if start.day() == now.day() {
                                    times.day += split;
                                }
                            }

                            times
                        }, 
                        _ => Times{ day: 0, week: 0, month: 0, total: 0, split: 0},
                    }
                }) 
        },
        Err(_) => Times{ day: 0, week: 0, month: 0, total: 0, split: 0}, 
    }
}


pub fn select_timer(dir:&str, name:&str) -> Times {
    let path = shellexpand::tilde(dir);
    let path = Path::new(path.as_ref());

    //check if all timers should be read
    match name {
        ALL_TIMERS  => {
            match fs::read_dir(path) {
                Ok(dir) => {
                    dir.map(|path| {
                        read_timer(path.unwrap().path().as_path())
                    })
                    .fold(Times { total: 0, split: 0, day: 0, week: 0, month: 0 }, |mut acc, file| {
                        acc.day += file.day;
                        acc.week += file.week;
                        acc.month += file.month;
                        acc.total += file.total;
                        acc.split += file.split;
                        acc
                    })
                },
                Err(_) => Times{ day: 0, week: 0, month: 0, total: 0, split: 0},
            }
        },
        _ => {read_timer(path.join(name).as_path())},
    }
}

pub fn read_timers(path:&str) -> Vec<String> {
    let path = shellexpand::tilde(path);
    let all = String::from(ALL_TIMERS);
    match fs::read_dir(path.as_ref()) {
        Ok(dir) => {
            let mut vec:Vec<String> = dir.map(|path| { String::from(path.unwrap()
                             .path()
                             .file_name()
                             .unwrap_or_default()
                             .to_string_lossy())})
                .collect();
            vec.push(all);
            vec
            
        },
        Err(_) => {fs::create_dir(path.as_ref()).unwrap(); vec![all]},
    }
}
