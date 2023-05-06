use chrono::prelude::*; 

fn main() {
    let time_str:String = Local::now().to_string();
    let time:DateTime<Local> = time_str.parse().unwrap();


    println!("{}", time);
    println!("{}", Local::now());
}
