use super::*;

/// handles possible actions in list mode
pub fn list_mode(state:&mut AppState, action:ActionList) -> Control {
    use ActionList::*;
    match action {
        Cancel  => {
            // change state
            state.mode = Mode::Normal;
            // send to ui
            state.sender.send(Event::TimersClose).unwrap();
            state.sender.send(Event::LegendUpdate(get_legend(&state.mode))).unwrap();
        },
        Up      => {
            // change state
            if state.selection > 0 { state.selection -= 1; }
            // send to ui
            state.sender.send(Event::TimersSelect(state.selection)).unwrap();
        },
        Down    => {
            // change state
            if state.selection + 1 < state.timers.len() { state.selection += 1; }
            // send to ui
            state.sender.send(Event::TimersSelect(state.selection)).unwrap();
        },
        Confirm => {
            // change state
            state.mode          = Mode::Normal;
            state.timer.name    = state.timers.get(state.selection).unwrap_or(&String::from("")).clone();
            state.timer.times   = files::read_timer(&state.path, &state.timer.name);
            // send to ui
            state.sender.send(Event::TimersSelect(0)).unwrap();
            state.sender.send(Event::TimersClose).unwrap();
            state.sender.send(Event::NameTick(state.timer.get_name())).unwrap();
            state.sender.send(Event::NameClose).unwrap();
            state.sender.send(Event::Tick(state.timer.get_clock())).unwrap();
            state.sender.send(Event::LegendUpdate(get_legend(&state.mode))).unwrap();
        },
        LegendToggle            => {state.sender.send(Event::LegendToggle).unwrap()}
    }
    Control::Continue
}
