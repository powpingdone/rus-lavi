// deviant.rs: the main logic for finding the least average image

use crate::imgload;
use itertools::multizip;
use linya::{Bar, Progress};
use std::sync::Arc;
use std::collections::VecDeque;
use parking_lot::Mutex;

// computes the least average image
pub fn least_average_img(imgs: Vec<String>, res: (u32, u32), largest: String, verbose: bool) -> Vec<(u8, u8, u8)> {
    // setup mutex Vecs
    constrain_mem();
    let img_main_master: Arc<Vec<Mutex<(u8, u8, u8)>>> = Arc::new(
        {
            let img = imgload::open_image(&largest, &res, verbose.clone()).unwrap();
            img.iter().map(|x| Mutex::new(*x)).collect()
        }
    );

    let mut dists_tmp: Vec<Mutex<f32>> = Vec::new();
    for pix in img_main_master.clone().iter() {
        dists_tmp.push(Mutex::new(dist(&pix.lock(), &(127, 127, 127))));
    }
    let dists_master: Arc<Vec<Mutex<f32>>> = Arc::new(dists_tmp);

    // setup threads
    let mut thread_pool = Vec::new();
    let img_queue_master = Arc::new(Mutex::new(VecDeque::from(imgs)));
    let prog = Arc::new(Mutex::new(Progress::new()));
    let bar: Arc<Bar> = Arc::new(
        prog.clone().lock()
        .bar(img_queue_master.clone().lock().len(), "least averaging images")
    );

    // run threads
    for _ in 0..num_cpus::get_physical() {
        // arc duplication boilerplate
        let img_queue = img_queue_master.clone();
        let img_main = img_main_master.clone();
        let dists_master = dists_master.clone();
        let self_bar = bar.clone();
        let self_prog = prog.clone();
        let largest_img = largest.clone(); // this clones the string, it is not Arc

        // start thread
        thread_pool.push(std::thread::spawn(move || {
            loop {
                let poss_img = { img_queue.lock().pop_front() };
                if let Some(img) = poss_img {
                    if img != largest_img {
                        if let Ok(img_loaded) = imgload::open_image(&img, &res, verbose.clone()) {
                            least_avg_thread(&img_main, &dists_master, img_loaded);
                        }
                    }
                    self_prog.lock().inc_and_draw(&self_bar, 1);
                }
                else {
                    break;
                }
            }
        }));
    }

    // wait for them to finish processing
    for t_join in thread_pool {
        t_join.join().unwrap();
    }

    // return non mutexed vec
    Arc::clone(&img_main_master).iter().map(|x| x.lock().clone()).collect()
}

// thread function
fn least_avg_thread(img_main: &Vec<Mutex<(u8, u8, u8)>>, dists: &Vec<Mutex<f32>>, img: Vec<(u8, u8, u8)>) {
    for (base, merge, old_dist) in multizip((img_main.iter(), img.iter(), dists.iter())) {
        let mut old_dist = old_dist.lock();
        let mut base = base.lock();
        let new_dist = dist(&merge, &base);

        // update references
        if new_dist > *old_dist {
            *base = *merge;
            *old_dist = new_dist;
        }
    }
}

// distance function
fn dist(merge: &(u8, u8, u8), base: &(u8, u8, u8)) -> f32 {
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


// util function to divide but to not div by zero, instead dividing by one
fn div_no_zero(dividend: &u8, divisor: &u8) -> f32 {
    let dividend: f32 = (*dividend).into();
    let full_div: f32 = (*divisor).into();
    dividend / ( if *divisor == 0 {1.0} else {full_div} )
}

// glibc configuration to not eat ram like chrome
#[cfg(target_os = "linux")]
fn constrain_mem() {
    extern crate libc;
    unsafe {
        libc::mallopt(libc::M_ARENA_MAX, 1);
        libc::mallopt(libc::M_ARENA_TEST, 1);
    }
}

#[cfg(not(target_os = "linux"))]
fn constrain_mem() {}
