use super::*;

/// handles possible actions in naming mode
pub fn rename_mode(state:&mut AppState, action:ActionName) -> Control {
    use ActionName::*;
    match action {
        Cancel  => {
            // change state 
            state.timer.name = state.prev_name.clone();
            state.mode       = Mode::Normal;
            // send to ui
            state.sender.send(Event::NameClose).unwrap();
            state.sender.send(Event::LegendUpdate(get_legend(&state.mode))).unwrap();
        },
        Confirm => {
            // change state
            state.mode = Mode::Normal;
            state.timer.times = files::read_timer(&state.path, &state.timer.name);
            //send to ui
            state.sender.send(Event::Tick(state.timer.get_clock())).unwrap();
            state.sender.send(Event::NameClose).unwrap();
            state.sender.send(Event::LegendUpdate(get_legend(&state.mode))).unwrap();
        },
        Delete  => {
            // change state
            state.timer.name.pop();
            // send to ui
            state.sender.send(Event::NameTick(state.timer.get_name())).unwrap();
        },
        Type(c) => {
            // change state
            state.timer.name.push(c);
            // send to ui
            state.sender.send(Event::NameTick(state.timer.name.clone())).unwrap();
        },
    }
    Control::Continue
}


