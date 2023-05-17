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
- [X] group timers(in directory) 
- [ ] split modes into multiple files
- [ ] cleanup state struct
- [X] rename timeview and timemode into timeframe, and timeaccumulated
- [ ] legend showing keybindings (the ones in usage)
