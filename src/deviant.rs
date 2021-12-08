// deviant.rs: the main logic for finding the least average image

extern crate itertools;
use crate::imgload;
use itertools::multizip;
use linya::{Bar, Progress};
use std::convert::TryInto;

// computes the least average image
pub fn least_average_arr(imgs: Vec<String>, res: (u32, u32), largest: String, verbose: bool) -> Vec<(u8, u8, u8)> {
    let mut img_main = imgload::open_image(&largest, &res, verbose.clone()).unwrap();
    let length: u64 = u64::from(res.0) * u64::from(res.1);
    let mut dists: Vec<f64> = Vec::new();
    dists.resize(length.try_into().unwrap(), 0.0);

    // main loop
    let mut prog = Progress::new();
    let bar: Bar = prog.bar(imgs.len(), "least average img");
    for img in imgs.iter() {
        if *img != largest {
            if let Ok(img_merge) = imgload::open_image(img, &res, verbose.clone()) {
                // for each value in each vector
                for (base, merge, dist) in multizip((img_main.iter_mut(), img_merge.iter(), dists.iter_mut())) {
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
                }
            }
        }
        prog.inc_and_draw(&bar, 1);
    }
    img_main
}


fn div_no_zero(dividend: &u8, divisor: &u8) -> f64 {
    let dividend: f64 = (*dividend).into();
    let full_div: f64 = (*divisor).into();
    dividend / ( if *divisor == 0 {1.0} else {full_div} )
}

