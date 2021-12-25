// imgload.rs: img handling tools

use crate::args;
extern crate image;
use image::Pixel;
use std::sync::Arc;

pub fn find_largest_resolution(imgs: &Vec<String>,
                               argstruct: &args::ParsedArgs)
                               -> ((u32, u32), Result<String, String>) {
    // the user specified the "largest" image as a base
    if let Some(img_name) = &argstruct.largest {
        (
            image::image_dimensions(img_name).expect("base image is invalid"),
            Ok(img_name.clone())
        )
    }
    // we will do it ourselves
    else {
        let size = largest_size(imgs, argstruct);

        if argstruct.verbose { println!("using size {}x{}", size.0, size.1); };
        (
            size,
            {
                let mut ret = Err("Cannot find image with resolution".to_string());
                for img in imgs.iter() {
                    let poss_dims = image::image_dimensions(img);
                    if let Ok(dims) = poss_dims {
                        if dims == size {
                           ret = Ok(img.to_string());
                           break;
                        }
                    }
                }

                if argstruct.verbose {
                    println!("{}",
                        {
                            if let Ok(ref bop) = ret {
                                format!("found largest img: {}", bop)
                            }
                            else {
                                format!("could not find largest image")
                            }
                        }
                    );
                }

                ret
            }
        )
    }
}

fn largest_size(imgs: &Vec<String>,
                argstruct: &args::ParsedArgs)
                -> (u32, u32) {

    // this first map gathers all image dimensions possible,
    // on error it will output a (0,0) tuple
    if argstruct.verbose { println!("checking {} images", imgs.len()) };
    imgs.iter().map(|img_name| {
        if argstruct.verbose { print!("checking {}...", img_name) };
        let image_in = image::image_dimensions(img_name);
        match image_in {
            Ok(image_good) => {
                if argstruct.verbose { println!("good! got {:?}", image_good); }
                image_good
            },
            Err(image_bad) => {
                if argstruct.verbose { println!("err: {}", image_bad.to_string()); }
                (0, 0)
            }
        }
    })
    // then fold will then find the highest
    // resolution by comparing dimentions
    .fold((0, 0), |highest_res, res| {
        if res.0 > highest_res.0 && res.1 > highest_res.1 {
            res
        }
        else {
            highest_res
        }
    })
}

// create semi-flattened array from image
pub fn open_image<'a>(img: &'a String,
                      size: &'a(u32, u32),
                      argstruct: &'a args::ParsedArgs)
                      -> Result<Vec<(u8, u8, u8)>, String> {

    // try opening image
    if argstruct.verbose { println!("opening img {}", img); }
    let img_object = image::open(img);
    if let Err(imgerr) = img_object {
        return Err(imgerr.to_string());
    }

    // transform image into standard size
    let img_object = {
        let i = img_object.unwrap();
        if image::image_dimensions(img).unwrap() != *size {
            i.resize_exact(size.0, size.1, image::imageops::FilterType::Lanczos3)
        }
        else { i }
    };

    // return the object
    Ok(
        img_object.into_rgb8().pixels().map(|x| {
            let pixel = x.channels4();
            (pixel.0, pixel.1, pixel.2)
        }).collect()
    )
}

// writes out semi-flattened array
pub fn write_image(size: (u32, u32),
                   data: Vec<(u8, u8, u8)>,
                   argstruct: Arc<args::ParsedArgs>)
                   -> Result<(), String> {

    if argstruct.verbose { println!("writing out {}", argstruct.output.as_ref().unwrap()); }

    // flatten array from (u8, u8, u8) to u8, u8, u8
    let mut flattened = Vec::new();
    for pixel in data {
        flattened.push(pixel.0);
        flattened.push(pixel.1);
        flattened.push(pixel.2);
    }

    // write out "raw data"
    if let Err(reterr) = image::save_buffer(argstruct.output.as_ref().unwrap(), &flattened, size.0, size.1, image::ColorType::Rgb8) {
        return Err(reterr.to_string());
    }
    Ok(())
}
