
use std::env;
use getopts::Occur;
use args::Args;


pub fn getargs() -> Result<Args, &'static str> {
    let mut args = Args::new("wordle-art", "Wordle art generator, takes an image and today's answer and outputs a series of guesses to produce that image");
    // Input options
    args.option("a", "answer", "The solution to today's Wordle", "ANS", Occur::Req, None);
    args.option("i", "img", "The input image to convert", "IMG", Occur::Req, None);
    args.option("d", "dict", "Optional - Dictionary of allowed words, default is the dict.txt provided", "DICT", Occur::Optional, Some(String::from("dict.txt")));
    // Transform options
    args.option("t", "trans", "Optional - Transform steps, see readme, default is no transformation", "TRANS", Occur::Optional, Some(String::from(" ")));
    // Output options
    args.option("o", "out", "Output file", "OUT", Occur::Req, None);
    args.option("u", "hint-out", "Optional - Hint output type, a=all hints, f=first hint, r=random hint, default is first", "TYPE", Occur::Optional, Some(String::from("f")));
    args.flag("n", "nope", "Flag - When included then no output will be writen if any rows fail to find a hint");
    args.flag("h", "help", "Flag - Print this help message");

    // Check for help argument
    let v: Vec<String> = env::args().collect();
    for s in v {
        if s.eq("-h") || s.eq("--help") {
            println!("{}", args.full_usage());
        }
    }
    
    match args.parse_from_cli() {
        Ok(()) => Ok(args),
        Err(err) => {
            println!("{}", err);
            Err("Error parsing arguments")
        }
    }
}

