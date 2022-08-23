# Wordle-Art-Generator
Command-line program for converting an artwork to a series of wordle guesses

# Compiling
Compile with cargo build --release

# Arguments
Required fields:

-a, --answer ANS    The solution to today's Wordle

-i, --img IMG       The input image to convert

-o, --out OUT       Output file

Optional fields:
-d, --dict DICT     Dictionary of allowed words, default is the dict.txt provided
-t, --trans TRANS   Transform steps, see below, default is no transformation
-u, --hint-out TYPE Hint output type, a=all hints, f=first hint, r=random hint, default is first
-n, --nope          Flag - When included then no output will be writen if any rows fail to find a hint
-h, --help          Print this help message, only works if first argument

# Usage
Supply this program with this day's wordle answer, an image file of the art you want produced, and the output file and it will tell you the guesses you need to input to produce that image.

If the input image is an actuall image, the colors in the image will be treated as the various hint colors. High-Contrast mode and Dark/Light themed colors are supported and will be detected automatically.
The input image can also just be a text file (detected automatically), in which case each line of text will be interpreted as a row of the image. 'X' characters are correct guesses (green), '~' characters are incorrect positions (yellow), and '-' is an incorrect guess (grey).

