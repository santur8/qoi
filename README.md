# qoi
A PNG to QOI compressor

How to build and run:
	To build the project, use "cargo build"
	To run the project, use "cargo run <file operation> <file>
	
The file operations are "encode" and "decode", which encode PNG files as QOI files 
and vice versa. 

The image to use must be placed in the images directory. Make sure to run the project in the 
root project directory, since the code uses the images/ path to find the specified files.
