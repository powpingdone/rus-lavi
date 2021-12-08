// deviant.rs: the main logic for finding the least average image

use crate::imgload;
use itertools::multizip;
use linya::{Bar, Progress};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

// computes the least average image
pub fn least_average_img(imgs: Vec<String>, res: (u32, u32), largest: String, verbose: bool) -> Vec<(u8, u8, u8)> {
    // setup mutex Vecs
    let img_main: Vec<Mutex<(u8, u8, u8)>> =
        imgload::open_image(&largest, &res, verbose.clone()).unwrap()
        .iter().map(|x| Mutex::new(*x)).collect();

    let img_length: u64 = u64::from(res.0) * u64::from(res.1);
    let mut dists: Vec<Mutex<f64>> = Vec::new();
    for _ in 0..img_length {
        dists.push(Mutex::new(0.0));
    }

    // setup threads
    let mut thread_pool = Vec::new();
    let mut img_queue = VecDeque::from(imgs);
    let prog = Mutex::new(Progress::new());
    let bar: Bar = prog.lock().unwrap().bar(imgs.len(), "least averaging images");
    for _ in 0..num_cpus::get() {
        thread_pool.push(std::thread::spawn(|| {
            while let Some(img) = img_queue.pop_front() {

            }
        }));
    }


    // main loop
    /*for img in imgs.iter() {
        if img != largest {
            if let Ok(img_merge) = imgload::open_image(img, &res, verbose.clone()) {
                // for each value in each vector
                for (base, merge, dist, set) in multizip((img_main.iter(), img_merge.iter(), dists.iter(), 0..img_length)) {
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
    }*/


    // return non mutexed vec
    img_main.iter().map(|x| x.lock().unwrap().clone()).collect()
}

fn least_avg_thread() {}

// util function to divide but to not div by zero, instead dividing by one
fn div_no_zero(dividend: &u8, divisor: &u8) -> f64 {
    let dividend: f64 = (*dividend).into();
    let full_div: f64 = (*divisor).into();
    dividend / ( if *divisor == 0 {1.0} else {full_div} )
}

