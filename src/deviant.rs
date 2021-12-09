// deviant.rs: the main logic for finding the least average image

extern crate itertools;
use crate::imgload;
use itertools::multizip;
use linya::{Bar, Progress};

// computes the least average image
pub fn least_average_arr(imgs: Vec<String>, res: (u32, u32), largest: String, verbose: bool) -> Vec<(u8, u8, u8)> {
    let mut img_main = imgload::open_image(&largest, &res, verbose.clone()).unwrap();
    let mut dists: Vec<f64> = Vec::new();
    // compute initial distance
    for pix in img_main.iter() {
        dists.push(dist(&pix, &(127, 127, 127)));
    }

    // main loop
    let mut prog = Progress::new();
    let bar: Bar = prog.bar(imgs.len(), "least average img");
    for img in imgs.iter() {
        if *img != largest {
            if let Ok(img_merge) = imgload::open_image(img, &res, verbose.clone()) {
                // for each value in each vector
                for (base, merge, old_dist) in multizip((img_main.iter_mut(), img_merge.iter(), dists.iter_mut())) {
                    let new_dist: f64 = dist(merge, base);

                    // update references
                    if new_dist > *old_dist {
                        *base = *merge;
                        *old_dist = new_dist;
                    }
                }
            }
        }
        prog.inc_and_draw(&bar, 1);
    }
    img_main
}

fn dist(merge: &(u8, u8, u8), base: &(u8, u8, u8)) -> f64 {
    /*
        distance formula is the "color ratio" where
        merge = m -> the new image to use
        base = s -> the basis image
        red = r
        green = g
        blue = b

        if a divisor is zero, it will automatically become one
        sqrt( (m.r / m.g - s.r / s.g)^2 + 2 * (m.r / m.b - s.r / s.b)^2 + (m.g / m.b - s.g / s.b)^2) )
    */
    (
        (div_no_zero(&merge.0, &merge.1) - div_no_zero(&base.0, &base.1)).powi(2) +
        2.0 * (div_no_zero(&merge.0, &merge.2) - div_no_zero(&base.0, &base.2)).powi(2) +
        (div_no_zero(&merge.1, &merge.2) - div_no_zero(&base.1, &base.2)).powi(2)
    ).sqrt()
}

fn div_no_zero(dividend: &u8, divisor: &u8) -> f64 {
    let dividend: f64 = (*dividend).into();
    let full_div: f64 = (*divisor).into();
    dividend / ( if *divisor == 0 {1.0} else {full_div} )
}

