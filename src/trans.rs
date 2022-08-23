use crate::parse::*;
use std::io::Write;
use std::fs;
use itertools::Itertools;



fn write_line(file: &mut fs::File, s: &str) -> Result<(), &'static str> {
    match file.write_all(s.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error writing to file")
    }
}
fn output(out_path: &str, list: &Vec<Vec<&Vec<char>>>, nope: bool, htype: char) -> Result<(), &'static str> {
    let mut file = match fs::File::create(out_path) {
        Ok(file) => file,
        Err(_) => return Err("Error oppening output file")
    };
    for row in list {
        // Get word to output on this line
        let s = if row.len() == 0 {
            // No words found match the hint for this row
            if nope == true {
                // Close & delete file and return
                fs::remove_file(out_path).expect("Error deleting file");
                return Ok(());
            } else {
                // -----
                "-----".to_string()
            }
        } else {
            match htype {
                'f' => {
                    // First element
                    vec_to_str(row[0])
                },
                'r' => {
                    // Random element
                    vec_to_str(row[rand::random::<usize>() % row.len()])
                },
                'a' => {
                    // List all elements, make a new vector and add all them to it
                    if row.len() == 1 {
                        vec_to_str(row[0])
                    } else {
                        let mut s: String = String::new();
                        let mut first: bool = true;
                        for v in row.iter() {
                            if !first {
                                s.push_str(", ");
                            }
                            s.push_str(&vec_to_str(v));
                            first = false;
                        }
                        s
                    }
                },
                _ => {return Err("Invalid guess output type")}
            }
        };
        // Write to file
        write_line(&mut file, &s)?;
        write_line(&mut file, "\n")?;

    }
    Ok(())
}


fn ins_name(s: String, c: char) -> String {
    let mut new = String::new();
    let mut split = s.split(".");
    match split.next() {
        Some(sec) => new.push_str(sec),
        None => {}
    };
    new.push(c);
    for sec in split {
        new.push('.');
        new.push_str(sec);
    }
    new
}


fn trans_flip(img: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new: Vec<Vec<char>> = Vec::new();
    // Flip each sub-vector
    for v in img {
        let mut row = v.clone();
        row.reverse();
        new.push(row);
    }
    new
}

fn trans_shift_left(img: &Vec<Vec<char>>, blank: char) -> Option<Vec<Vec<char>>> {
    // Returns None if already shifted as far as it can go
    let mut new: Vec<Vec<char>> = Vec::new();
    let mut allblank: bool = true;      // Could get stuck in infinite loop without this
    // Shift each sub-vector
    for v in img {
        let mut iter = v.iter();
        // Check leftmost value
        if *iter.next().unwrap() != blank {
            return None;
        }
        // Put remaining values in new row
        let mut row: Vec<char> = Vec::new();
        for c in iter {
            row.push(*c);
            if *c != blank {
                allblank = false;
            }
        }
        // Put rightmost character
        row.push(blank);
        new.push(row);
    }
    if allblank {
        None
    } else {
        Some(new)
    }
}

fn trans_shift_right(img: &Vec<Vec<char>>, blank: char) -> Option<Vec<Vec<char>>> {
    // Returns none if already shifted as far as it can go
    let mut new: Vec<Vec<char>> = Vec::new();
    let mut allblank: bool = true;
    // Shift each sub-vector
    for v in img {
        let mut row: Vec<char> = Vec::new();
        // Put leftmost character
        row.push(blank);
        // Put remaining characters
        for c in v.iter() {
            row.push(*c);
            if *c != blank {
                allblank = false;
            }
        }
        // Check rightmost value
        if row.pop() != Some(blank) {
            return None;
        }
        new.push(row);
    }
    if allblank {
        None
    } else {
        Some(new)
    }
}

fn trans_color(img: &Vec<Vec<char>>, chart: &Vec<&char>) -> Result<Vec<Vec<char>>, &'static str> {
    static CS: [char; 3] = ['X', '~', '-'];
    // Produces an image with the color shuffled
    let mut new: Vec<Vec<char>> = Vec::new();
    for v in img {
        let mut row: Vec<char> = Vec::new();
        for c in v.iter() {
            // Get index value of the character and replace with that specified by the input
            let ind = match CS.iter().position(|&r| r == *c) {
                Some(ind) => ind,
                None => return Err("Error during color transformation: Invalid color code")
            };
            row.push(*chart[ind]);
        }
        new.push(row);
    }
    Ok(new)
}


pub fn apply_trans_and_convert(ans: &Vec<char>, img: Vec<Vec<char>>, dict: &Vec<Vec<char>>, mut trans: String, out: &str, nope: bool, hout: char) -> Result<(), &'static str> {
    if trans.len() == 0 {
        // No more transformations to perform, can now convert this image
        let parsed = convertart(ans, &img, dict)?;
        output(out, &parsed, nope, hout)
    } else {
        // Get transformation to apply
        let t = trans.chars().next().unwrap();
        trans.remove(0);
        // Recursively call this function with all the possible applications of the transformation
        match t {
            's' => {
                // Shift left/right
                // Decide what the "background" color is
                let bg = img[0][0];
                // Shift left until not doable
                let mut new: Vec<Vec<char>> = img.clone();
                let mut newout: String = out.to_string();
                newout = ins_name(newout, '_');
                loop {
                    match trans_shift_left(&new, bg) {
                        Some(m) => {new = m;
                            newout = ins_name(newout, 'l');
                            apply_trans_and_convert(ans, new.clone(), dict, trans.clone(), &newout, nope, hout)?;},
                        None => break
                    };
                }
                let bg = img[0][img[0].len()-1];
                // Shift right until not doable
                let mut new: Vec<Vec<char>> = img.clone();
                let mut newout: String = out.to_string();
                newout = ins_name(newout, '_');
                loop {
                    match trans_shift_right(&new, bg) {
                        Some(m) => {new = m;
                            newout = ins_name(newout, 'r');
                            apply_trans_and_convert(ans, new.clone(), dict, trans.clone(), &newout, nope, hout)?;},
                        None => break
                    };
                }
            }
            'f' => {
                // Flip
                let newout: String = ins_name(out.to_string(), '_');
                let newout = ins_name(newout, 'f');
                apply_trans_and_convert(ans, trans_flip(&img), dict, trans.clone(), &newout, nope, hout)?;
            }
            'c' => {
                // Change color
                static ITEMS: [char; 3] = ['X', '~', '-'];
                for perm in ITEMS.iter().permutations(ITEMS.len()).unique() {
                    let mut newout: String = out.to_string();
                    newout = ins_name(newout, '_');
                    for c in &perm {
                        newout = ins_name(newout, **c);
                    }
                    apply_trans_and_convert(ans, trans_color(&img, &perm)?, dict, trans.clone(), &newout, nope, hout)?;
                }
            }
            ' ' => {
                // No transformation
            }
            _ => {
                // Invalid transformation character
                return Err("Invalid transformation character");
            }
        }
        apply_trans_and_convert(ans, img, dict, trans, out, nope, hout)?;
        return Ok(())
    }
        
}



