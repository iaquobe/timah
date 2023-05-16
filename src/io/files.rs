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

pub fn read_timer(path:&str, name:&str) -> Times {
    let path = shellexpand::tilde(path);
    let path = Path::new(path.as_ref()).join(name);
    let now = Local::now(); 

    match fs::read(path) {
        Ok(file) => {
            file.lines()
                .fold(Times::default(), |mut times, line| {
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
                        _ => Times::default(),
                    }
                }) 
        },
        Err(_) => Times::default(), 
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

pub fn read_all_timers(path:&str) -> Times {
    read_timers(path).iter()
        .map(|timer| read_timer(path, timer) )
        .fold(Times::default(), |sum, times| {
            sum + times
        })
}
