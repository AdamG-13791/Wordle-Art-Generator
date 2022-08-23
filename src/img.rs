use image::GenericImageView;

use crate::parse::*;


fn fmax3(f1: f64, f2: f64, f3: f64) -> f64 {
    if f1 > f2 {
        if f1 > f3 {f1}
        else {f3}
    } else {
        if f2 > f3 {f2}
        else {f3}
    }
}


// Convert rgb color to hue saturation and value
// (based on https://www.rapidtables.com/convert/color/rgb-to-hsv.html)
fn get_hue(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let rp: f64 = r as f64 / 255.0;
    let gp: f64 = g as f64 / 255.0;
    let bp: f64 = b as f64 / 255.0;
    let cmax = fmax3(rp, gp, bp);
    let cmin = fmax3(rp, gp, bp);
    let d = cmax - cmin;

    // Calculate hue
    let mut hue: f64 = if d == 0.0 {
        0.0
    } else if cmax == rp {
        60.0 * (((gp - bp) / d) % 6.0)
    } else if cmax == gp {
        60.0 * (((bp - rp) / d) + 2.0)
    } else {
        60.0 * (((rp - gp) / d) + 4.0)
    };
    if hue < 0.0 {
        hue += 360.0;
    }
    
    // Calculate saturation
    let sat: f64 = if cmax == 0.0 {
        0.0
    } else {
        d / cmax
    };

    // Calculate value
    let val: f64 = cmax;

    return (hue, sat, val);
}



pub fn loadimg(name: &str) -> Result<Vec<Vec<char>>, &'static str> {
    
    match image::open(name) {
        Ok(img) => {
            // Image succesfully opened
            let (w, h) = img.dimensions();
            let mut vimg: Vec<Vec<char>> = Vec::new();
            for y in 0..h {
                let mut row: Vec<char> = Vec::new();
                for x in 0..w {
                    let pxl = img.get_pixel(x, y);
                    
                    // Get the type of hint this pixel represents
                    let image::Rgba(data) = pxl;
                    let (h, _s, v) = get_hue(data[0], data[1], data[2]);
                    let c = if v < 0.4 {
                        '-' // Grey / Black / Write
                    } else if (h >= 90.0) && (h < 160.0) {
                        'X' // Green
                    } else if (h < 34.0) || (h >= 300.0) {
                        'X' // Red
                    } else {
                        '~' // Yellow / Blue / Cyan
                    };
                    row.push(c);
                }
                vimg.push(row);
            }

            Ok(vimg)
        },
        Err(_) => {
            // Failed to open as image, try to open as text file
            loadwords(name, "Failed to open image file")
        }   
    }
    
}



