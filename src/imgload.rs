// imgload.rs: img handling tools

extern crate image;

pub fn find_largest_resolution(imgs: &Vec<String>) -> (u32, u32) {
    imgs.iter().map(|img_name| {
        let image_in = image::image_dimensions(img_name);
        if let Ok(image_good) = image_in {
            return image_good;
        }
        (0, 0)
    }).fold((0, 0), |highest_res, res| {
        if res.0 > highest_res.0 && res.1 > highest_res.1 {
            return res;
        }
        highest_res
    })
}
