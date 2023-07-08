use std::time::{Duration,SystemTime};
use chrono::prelude::*;
use super::{AppState,Event};

mod times;
mod timer;
mod clock;
mod pomodore;
pub use times::*;
pub use timer::*;
pub use clock::*;
pub use pomodore::*;

pub fn timer(state: &mut AppState) {
    // check if 1 second has elapsed
    if let Ok(elapsed) = state.timer.now.elapsed() {
        let elapsed_seconds = elapsed.as_secs(); 
        if let TimerState::Running = state.timer.state {
            // increase timer
            state.timer.tick(elapsed_seconds as i32, &state.sender);
            state.pomodore.tick(elapsed_seconds as i32, &state.sender);
        }
        state.timer.now = state.timer.now + Duration::from_secs(elapsed_seconds);
    }
}

