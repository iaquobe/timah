# Timah
Timah tracks the time you spend on different activities.
You can create timers with different names and simply press pause and play to
record the time. 

Say for instance you want to monitor how much you work, and how much time you
spend on your classes at university. Then you can create a timer for each of
those activities you want to monitor

[](./demo/timah.gif)


## Installation
```
git clone https://github.com/iaquobe/timah
cd timah 
cargo install --path .
```


## Usage
Timah has 3 modes: 

- Nomal: here you can start/stop the timer
- Rename: renames the current timer
- Open: shows list of saved timers you can choose to use

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
- readme with docs and gifs
- timer toggleable between showing slice or total
- timer reset 
- legend showing keybindings
