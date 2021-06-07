# tacview-splitter

a python program that splits a tacview file by coalition
 
## Usage
Put the executable and the tacview file you want split up in the same directory, and execute the program.

The program processes the first file it finds, therefore make sure to only have one tacview file in the directory.

Supports both zipped and ASCII tacview files (`.zip.acmi` and `.txt.acmi`).

## building
Requires Python 3.

`pyinstaller --onefile tacview-splitter.py`

