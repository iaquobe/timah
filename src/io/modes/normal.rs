
use super::*;

/// handles possible actions in normal mode
pub fn normal_mode(state:&mut AppState, action:ActionNormal) -> Control {
    use ActionNormal::*;
    match action {
        Quit                    => {return quit(state)},
        Pause                   => pause(state),
        Rename                  => rename(state),
        OpenList                => open_list(state),
        SwitchView              => switch_time_frame(state),
        SwitchTimerAccumulate   => switch_timer_accumulate(state),
    }
    Control::Continue
}


fn quit(state:&mut AppState) -> Control {
    // save timer if running
    if let TimerState::Running = state.timer.state {
        let end = Local::now();
        files::write_timer(&state.path, &state.timer.name, &state.timer.start, &end);
    }

    state.sender.send(Event::Quit).unwrap();
    return Control::Break;
}


fn pause(state:&mut AppState) {
    // change state
    state.timer.state = match state.timer.state {
        TimerState::Running => {
            let end = Local::now();
            files::write_timer(&state.path, &state.timer.name, &state.timer.start, &end);
            TimerState::Paused
        }, 
        TimerState::Paused  => {
            // reset time slice
            state.timer.times.day   += state.timer.times.split;
            state.timer.times.week  += state.timer.times.split;
            state.timer.times.month += state.timer.times.split;
            state.timer.times.total += state.timer.times.split;

            state.timer.total.day   += state.timer.times.split;
            state.timer.total.week  += state.timer.times.split;
            state.timer.total.month += state.timer.times.split;
            state.timer.total.total += state.timer.times.split;

            state.timer.times.split = 0; 

            state.timer.start = Local::now();
            TimerState::Running
        },};

    // update timer
    state.sender.send(Event::Tick(state.timer.get_clock())).unwrap();
}

fn rename(state:&mut AppState) {
    // change state
    state.mode       = Mode::Name;
    state.timer.name = String::from("");
    // send to ui
    state.sender.send(Event::NameOpen(state.timer.name.clone())).unwrap();
}

fn open_list(state:&mut AppState) {
    // change state
    state.mode      = Mode::List; 
    state.selection = 0;
    state.timers = files::read_timers(&state.path);
    // send to ui
    state.sender.send(Event::TimersOpen(state.timers.clone())).unwrap();
}

fn switch_time_frame(state:&mut AppState) {
    // change state
    use TimeFrame::*;
    state.timer.view = match state.timer.view {
        Total => Month,
        Month => Week,
        Week  => Day,
        Day   => Split,
        Split => Total,
    };
    // send to ui
    state.sender.send(Event::NameView(state.timer.get_view())).unwrap();
    state.sender.send(Event::Tick(state.timer.get_clock())).unwrap();
}

fn switch_timer_accumulate(state:&mut AppState) {
    // change state
    state.timer.mode = match state.timer.mode {
        TimerAccumulate::Total => TimerAccumulate::Timer,
        TimerAccumulate::Timer => {
            state.timer.total = read_all_timers(&state.path);
            TimerAccumulate::Total
        },
    };
    // send to ui
    state.sender.send(Event::NameTick(state.timer.get_name())).unwrap();
    state.sender.send(Event::NameClose).unwrap();
    state.sender.send(Event::Tick(state.timer.get_clock())).unwrap();
}
