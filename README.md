# Color Sort Puzzle Solver
It uses rust implementation of breadth first search to find the most optimal solution.
It uses python to input the starting position of the puzzle.

## Installation
To install just clone the repo and run build.sh.<br>
Note that you are in the root directory of the project before running the script.
<br><br>
Alternatively, you may
1. Compile the cargo project using release flag.
2. Move or copy the binary to the bin folder
## Usage
To run, type the command: python main.py && ./bin/sorter<br>
Or you may run the python file and then the rust binary.
Then you get the prompt to fill up the rows columns and heights of the bottles.<br>
After this a new screen pops up. There will be the bottles arranged all filled with 0.<br>
Use "h", "j", "k", "l" to move the cursor and "n" to select and "m" to undo.
The colors are replaced by numbers. Select the all the occurances of colors one by one.<br>
Finally the data is tranlated into a csv file. The header is ignored and the first value of last line is the number of empty bottles.<br>
In the output the numbering of bottles is in the form of left to right and then top to bottom. The first bottle is 1.

## Developer Notes
I know that there is a severe lack of error handling. I may add them but it is just a hobby project I built in my free time.<br>
I have used AI to review the code before publishing. But the code is written entirely by me.<br>
Adressing the elephant in the room, it is not conpatible for windows(afaik). You have to edit the file paths for it to work and the build script doesn't work. I may later use the cross platform version.
I am new to actually making something for others and is my first complete project on github. If you find the code more verbose, feel free to help me make the code more readable and idiomatic.
P.S. I don't know if there needs to be anything else in the README.md.
