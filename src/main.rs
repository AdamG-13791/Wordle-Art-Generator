mod parse;
mod img;
mod inp;
mod trans;

extern crate args;
extern crate getopts;

use crate::img::*;
use crate::parse::*;
use crate::inp::*;
use crate::trans::*;


fn process() -> Result<(), &'static str> {
    // Get command-line arguments
    let args = getargs()?;
    // Load input files
    let img = loadimg(&args.value_of::<String>("img").unwrap())?;
    let dict = loadwords(&args.value_of::<String>("dict").unwrap(), "Failed to load dictionary")?;
    let ans = str_to_vec(&args.value_of::<String>("answer").unwrap());
    let out = args.value_of::<String>("out").unwrap();
    apply_trans_and_convert(&ans, img, &dict, args.value_of::<String>("trans").unwrap(), &out, args.value_of("nope").unwrap(), args.value_of::<String>("hint-out").unwrap().chars().next().unwrap())
}



fn main() {
    match process() {
        Ok(()) => println!("Finished!"),
        Err(err) => println!("Error: {}", err)
    };
}




