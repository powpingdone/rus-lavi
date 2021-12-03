// imgload.rs: img handling tools

extern crate image;

pub fn find_largest_resolution(imgs: &Vec<String>) -> (u32, u32) {
    // this first map gathers all image dimentions possible,
    // on error it will output a (0,0) tuple
    imgs.iter().map(|img_name| {
        let image_in = image::image_dimensions(img_name);
        if let Ok(image_good) = image_in {
            return image_good;
        }
        (0, 0)
    })
    // this fold will then find the highest
    // resolution by comparing dimentions
    .fold((0, 0), |highest_res, res| {
        if res.0 > highest_res.0 && res.1 > highest_res.1 {
            return res;
        }
        highest_res
    })
}
