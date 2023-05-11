use std::path::Path;
use std::fs::{self,OpenOptions};
use std::io::{Write,BufRead};
use chrono::{Local,DateTime, Datelike};
use crate::io::timer::Times;

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


pub fn read_timer(dir:&str, name:&str) -> Times {
    // read file and add up all time splits 
    let now  = Local::now();
    let path = Path::new(dir).join(name);
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


pub fn read_timers(path:&str) -> Vec<String> {
    let path = shellexpand::tilde(path);
    match fs::read_dir(path.as_ref()) {
        Ok(dir) => {
            dir.map(|path| { String::from(path.unwrap()
                             .path()
                             .file_name()
                             .unwrap_or_default()
                             .to_string_lossy())})
                .collect()
        },
        Err(_) => {fs::create_dir(path.as_ref()).unwrap(); vec![]},
    }
}
