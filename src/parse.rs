use std::fs::File;
use std::io::{prelude::*, BufReader};



fn charind(mut c: char) -> Result<usize, &'static str> {
    c = c.to_lowercase().next().unwrap();
    match c >= 'a' && c <= 'z' {
        true => Ok(c as usize - 'a' as usize),
        false => Err("Error: character out of range (is not a normal letter)")
    }
}


// Count the amount of each character in a string
fn numchar(word: &Vec<char>) -> Result<Vec<u8>, &'static str> {
    let mut count: Vec<u8> = vec![0; 26];   // Vector which stores the occurances of each of the 26 letters
    for c in word {
        count[charind(*c)?] += 1;
    }
    Ok(count)
}


// Get the hints associated with a guess
fn gethint(guess: &Vec<char>, answer: &Vec<char>) -> Result<Vec<char>, &'static str> {
    if guess.len() != answer.len() {
        return Err("Error: compared words not the same length");
    }
    let ans_count: Vec<u8> = numchar(&answer)?;
    let mut gue_count = vec![0; 26];
    let mut hint = vec!['-'; guess.len()];
    // Start by only looking for correct (green) guesses
    for (i, c) in guess.iter().enumerate() {
        if answer[i] == *c {
            hint[i] = 'X';
            gue_count[charind(*c)?] += 1;
        }
    }
    // Then look at incorrect or misplaced guesses
    for (i, c) in guess.iter().enumerate() {
        // Skip if already processed as a correct guess
        if hint[i] == 'X' {continue;}
        let ind = charind(*c)?;
        gue_count[ind] += 1;
        // If the amount of this char discovered in the guess is more than in the answer, then it's
        // an incorrect guess (grey, default). Otherwise it's just misplaced (yellow)
        if gue_count[ind] <= ans_count[ind] {
            hint[i] = '~';
        }
    }
    Ok(hint)
}


fn hint_equ(s1: &Vec<char>, s2: &Vec<char>) -> bool {
    for (i, c) in s1.iter().enumerate() {
        if *c != s2[i] {
            return false;
        }
    }
    true
}


// Returns a list of guesses which result in the requested pattern
fn listguesses<'a>(answer: &Vec<char>, hint: &Vec<char>, wlist: &'a Vec<Vec<char>>) -> Result<Vec<&'a Vec<char>>, &'static str> {
    // Brute-force produce hints for each accepted ford and check that with the desired hint
    let mut lis: Vec<&Vec<char>> = Vec::new();
    for s in wlist.iter() {
        let h = gethint(s, answer)?;
        if hint_equ(hint, &h) {
            lis.push(s);
        }
    }
    Ok(lis)
}


// Take a full artwork and return a list of possible guesses for each row
pub fn convertart<'a>(answer: &Vec<char>, art: &Vec<Vec<char>>, wlist: &'a Vec<Vec<char>>) -> Result<Vec<Vec<&'a Vec<char>>>, &'static str> {
    let mut out: Vec<Vec<&Vec<char>>> = Vec::new();
    for hint in art {
        out.push(listguesses(answer, hint, wlist)?);
    }
    Ok(out)
}


// Load words from dictionary, or any list of words into a vector list
pub fn loadwords(path: &str, err: &'static str) -> Result<Vec<Vec<char>>, &'static str> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(err)
    };
    let reader = BufReader::new(file);
            
    let mut dict: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err(err)
        };
        dict.push(str_to_vec(&line));
    }
    Ok(dict)
}


// Convert string to vector of characters
pub fn str_to_vec(s: &str) -> Vec<char> {
    let mut v: Vec<char> = Vec::new();
    for c in s.chars() {
        v.push(c);
    }
    v
}

// Convert vector of characters to string
pub fn vec_to_str(v: &Vec<char>) -> String {
    let mut s: String = String::new();
    for c in v.iter() {
        s.push(*c);
    }
    s
}


