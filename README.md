# Wordle-Art-Generator
Command-line program for converting an artwork to a series of wordle guesses

# Compiling
Compile with "cargo build --release"  
The program is called "wordle-art" and will be located in target/release

# Arguments
Required fields:

-a, --answer (ANS): The solution to today's Wordle  
-i, --img (IMG): The input image to convert  
-o, --out (OUT): Output file

Optional fields:

-d, --dict (DICT): Dictionary of allowed words, default is the dict.txt provided  
-t, --trans (TRANS): Transform steps, see below, default is no transformation  
-u, --hint-out (TYPE): Hint output type, a=all hints, f=first hint, r=random hint, default is first  
-n, --nope: Flag - When included then no output will be writen if any rows fail to find a hint  
-h, --help: Print the help message, only works if first argument

# Usage
Supply this program with this day's wordle answer, an image file of the art you want produced, and the output file and it will tell you the guesses you need to input to produce that image.

## More about the image
If the input image is an actuall image, the colors in the image will be treated as the various hint colors. High-Contrast mode and Dark/Light themed colors are supported and will be detected automatically.  
The input image can also just be a text file (detected automatically), in which case each line of text will be interpreted as a row of the image. 'X' characters are correct guesses (green), '~' characters are incorrect positions (yellow), and '-' is an incorrect guess (grey).

## More about the output
By default, each line of the output gives only one guess you should enter, that being the first guess it finds that works. This behavior can be changed with -u and can also output a random guess it finds, or output a big list of all the guesses it's found.  

## More about transformations
It's entirely bossible that one or more rows of your image will not be able to be generated with a valid guess, in which case that line of the output will simply be '-----'. A simple way to deal with this is to simply modify the input image until it produces a result you want, but this can be tedious, especially with particularly stuborn images. But fear not! This program has a build-in way of dealing with this. Using -t (transformation), the program will automatically and recursively apply a series of transformations to the input image and attempt to produce outputs for each resulting image. Valid transformations are shifting the image left/right ('s'), fliping the image in place ('f'), and shuffling around the colors ('c'). The program will try all combinations of the transformations, so saying -t fs will try shifing both the fliped and unfliped image.  
If transformations are used it is recomended to also use the -n flag to prevent output if the image cannot be formed.

# Todo/Bugs
* Image input has not been thoroughly tested
* It'd be neat to add a rotation transformation
* Is Wordle even still popular?
