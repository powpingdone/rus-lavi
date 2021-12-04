// imgload.rs: img handling tools

extern crate image;
use std::convert::TryInto;

pub fn find_largest_resolution(imgs: &Vec<String>) -> ((u32, u32), Result<String, String>) {
    // this first map gathers all image dimentions possible,
    // on error it will output a (0,0) tuple
    let size = imgs.iter().map(|img_name| {
        let image_in = image::image_dimensions(img_name);
        if let Ok(image_good) = image_in {
            image_good
        } else {
            (0, 0)
        }
    })
    // then fold will then find the highest
    // resolution by comparing dimentions
    .fold((0, 0), |highest_res, res| {
        if res.0 > highest_res.0 && res.1 > highest_res.1 {
            res
        } else {
            highest_res
        }
    });
    // find the image itself
    (size, {
        let mut bub = Err("Cannot find image with resolution".to_string());
        for img in imgs.iter() {
            let poss_dims = image::image_dimensions(img);
            if let Ok(dims) = poss_dims {
                if dims == size {
                   bub = Ok(img.to_string());
                }
            }
        }
        bub
    })
}

// create flattened array from image
pub fn open_image<'a>(img: &'a String, size: &'a(u32, u32)) -> Vec<(u8, u8, u8)> {
    let img_object = image::open(img).unwrap()
                .resize_exact(size.0, size.1, image::imageops::FilterType::Lanczos3);
    let img = img_object.as_bytes();
    let mut ret: Vec<(u8, u8, u8)> = Vec::new();
    for x in 0..(size.0 * size.1){
        let i: usize = x.try_into().unwrap();
        let new_tuple: (u8, u8, u8) = (img[3*i].clone(), img[3*i+1].clone(), img[3*i+2].clone());
        ret.push(new_tuple);
    }
    ret
}

