# Timah
Timah lets you create timers for different activities and track the time you
spend on them.

It saves the timestamps in `~/.cache/timah/` in an editable format.

![demo](./demo/timah.gif)


## Installation
First you need to have curses and curses devel installed, then you can run: 
```
git clone https://github.com/iaquobe/timah
cd timah 
cargo install --path .
```

Once installed you can start timah with `timah`

## Usage
Timah has 3 modes: 

- Nomal: move to other modes or start/pause timer
- Rename: rename current timer
- Open: show list to open saved timer

### Normal 
- `<space>`: starts/pauses timer: saves timestamps 
- `o`: open list of timers
- `n`: rename the current timer
- `q`: quit the program

### Rename
- `<enter>`: confirm new name
- `<esc>`: cancel new name

### Open
- `<enter>`: confirm timer
- `j/k`: down/up
- `q/<esc>`: cancel open




# TODO: 
- [X] timer toggle between (total, since reset, day, slice)
- [X] timer reset(save reset in cache, which can be used for instance to reset every week to track the time in one week)
- [ ] group timers(in directory) 
- [ ] legend showing keybindings (the ones in usage)


# design changes
- toggle between showing all is not a separate timer
- for this upon startup read all timers
- when opening list, then read all files again, in case new timer was added
- when opening timer, then read time again, in case times have changed
- for this save list of timers, with times loaded
