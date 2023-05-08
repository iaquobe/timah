use std::path::Path;
use std::fs::{self,OpenOptions};
use std::io::{Write,BufRead};
use chrono::{Local,DateTime};

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


pub fn read_timer(dir:&str, name:&str) -> i32 {
    // read file and add up all time splits 
    let path = Path::new(dir).join(name);
    match fs::read(path) {
        Ok(file) => {
            file.lines()
                .fold(0, |sec, line| {
                    let line = line.unwrap();
                    let mut iter = line.split("---");
                    // try to read dates, otherwise its a reset char
                    match (iter.next().unwrap().parse::<DateTime<Local>>(), iter.next().unwrap().parse::<DateTime<Local>>()) {
                        (Ok(start), Ok(end)) => {
                            sec + (end - start).num_seconds()
                        }, 
                        _ => 0,
                    }
                }) as i32
        },
        Err(_) => 0, 
    }
}


pub fn read_timers(path:&str) -> Vec<String> {
    fs::read_dir(shellexpand::tilde(path).as_ref()).unwrap()
        .map(|path| {
            String::from(path.unwrap()
                         .path()
                         .file_name()
                         .unwrap_or_default()
                         .to_string_lossy())})
        .collect()
}
