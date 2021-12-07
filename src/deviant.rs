// deviant.rs: the main logic for finding the least average image

extern crate itertools;
use crate::imgload;
use itertools::multizip;
use std::convert::TryInto;
use linya::{Bar, Progress};

// computes the least average image
pub fn least_average_arr<'a>(imgs: &'a Vec<String>, res: &'a(u32, u32), largest: &'a String, verbose: bool) -> Vec<(u8, u8, u8)> {
    let mut img_main = imgload::open_image(&largest, &res, verbose.clone()).unwrap();
    let length: usize = (res.0 * res.1).try_into().unwrap();
    let mut dists: Vec<f64> = Vec::new();
    dists.resize(length, 0.0);

    // main loop
    for img in imgs.iter() {
        if img != largest {
            if let Ok(img_merge) = imgload::open_image(img, &res, verbose.clone()) {
                let mut prog = Progress::new();
                let bar: Bar = prog.bar(length, img);

                // for each value in each vector
                for (base, merge, dist, set) in multizip((img_main.iter_mut(), img_merge.iter(), dists.iter_mut(), 0..length)) {
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
                    let new_dist: f64 = (
                        (div_no_zero(&merge.0, &merge.1) - div_no_zero(&base.0, &base.1)).powi(2) +
                        2.0 * (div_no_zero(&merge.0, &merge.2) - div_no_zero(&base.0, &base.2)).powi(2) +
                        (div_no_zero(&merge.1, &merge.2) - div_no_zero(&base.1, &base.2)).powi(2)
                    ).sqrt();

                    // update references
                    if new_dist > *dist {
                        *base = *merge;
                        *dist = new_dist;
                    }
                    prog.set_and_draw(&bar, set);
                }
            }
        }
    }
    img_main
}


fn div_no_zero(dividend: &u8, divisor: &u8) -> f64 {
    let dividend: f64 = (*dividend).into();
    let full_div: f64 = (*divisor).into();
    dividend / ( if *divisor == 0 {1.0} else {full_div} )
}

