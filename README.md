# hugis-rs
A terminal program that takes in commands and performs various actions such as creating new windows and shapes, filling the window with a character, printing the current state of the window, and replacing characters in the window.

## Commands
- **'clear'**: Clears the terminal screen
- **'new window [width] [height]'**: Creates a new window with the specified width and height
- **'new shape [shape] [size]'**: Creates a new shape and adds it to the list of shapes
  - Available shapes **'circle'**, **'square'**
- **'list'**: Lists all shapes in the current list of shapes
- **'fill [character]'**: Fills the current window with the specified character
- **'print'**: Prints the current state of the window
- **'draw [shape index] [x coordinate] [y coordinate] [character]'**: Draws the specified shape at the given coordinates in the window, using the specified character.
- **'replace [character] [replacement character]'**: Replaces all instances of the specified shape in the window with the given character.
- **'resize [width] [height]'**: Resizes the current window to the specified width and height
