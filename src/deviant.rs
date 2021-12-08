// deviant.rs: the main logic for finding the least average image

use crate::imgload;
use itertools::multizip;
use linya::{Bar, Progress};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

// computes the least average image
pub fn least_average_img(imgs: Vec<String>, res: (u32, u32), largest: String, verbose: bool) -> Vec<(u8, u8, u8)> {
    // setup mutex Vecs
    let img_main_master: Arc<Vec<Mutex<(u8, u8, u8)>>> =
        Arc::new(imgload::open_image(&largest, &res, verbose.clone()).unwrap()
        .iter().map(|x| Mutex::new(*x)).collect());

    let img_length: u64 = u64::from(res.0) * u64::from(res.1);
    let mut dists_tmp: Vec<Mutex<f64>> = Vec::new();
    for _ in 0..img_length {
        dists_tmp.push(Mutex::new(0.0));
    }
    let dists_master: Arc<Vec<Mutex<f64>>> = Arc::new(dists_tmp);

    // setup threads
    let mut thread_pool = Vec::new();
    let img_queue_master = Arc::new(Mutex::new(VecDeque::from(imgs)));
    let prog = Arc::new(Mutex::new(Progress::new()));
    let bar: Arc<Bar> = Arc::new(
        Arc::clone(&prog).lock().unwrap()
        .bar(Arc::clone(&img_queue_master).lock().unwrap().len(), "least averaging images"));
    for _ in 0..num_cpus::get_physical() {
        // arc duplication boilerplate
        let img_queue = Arc::clone(&img_queue_master);
        let img_main = Arc::clone(&img_main_master);
        let dists_master = Arc::clone(&dists_master);
        let self_bar = Arc::clone(&bar);
        let self_prog = Arc::clone(&prog);
        let largest_img = largest.clone();

        // start thread
        thread_pool.push(std::thread::spawn(move || {
            while let Some(img) = img_queue.lock().unwrap().pop_front() {
                if img != largest_img {
                    if let Ok(img_loaded) = imgload::open_image(&img, &res, verbose.clone()) {
                        least_avg_thread(&img_main, &dists_master, img_loaded);
                    }
                }
                self_prog.lock().unwrap().inc_and_draw(&self_bar, 1);
            }
        }));
    }

    // wait for them to finish processing
    for t_join in thread_pool {
        t_join.join().unwrap();
    }

    // return non mutexed vec
    Arc::clone(&img_main_master).iter().map(|x| x.lock().unwrap().clone()).collect()
}

fn least_avg_thread(img_main: &Vec<Mutex<(u8, u8, u8)>>, dists: &Vec<Mutex<f64>>, img: Vec<(u8, u8, u8)>) {
    for (base, merge, dist) in multizip((img_main.iter(), img.iter(), dists.iter())) {
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
        let mut dist = dist.lock().unwrap();
        let mut base = base.lock().unwrap();
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

// util function to divide but to not div by zero, instead dividing by one
fn div_no_zero(dividend: &u8, divisor: &u8) -> f64 {
    let dividend: f64 = (*dividend).into();
    let full_div: f64 = (*divisor).into();
    dividend / ( if *divisor == 0 {1.0} else {full_div} )
}

