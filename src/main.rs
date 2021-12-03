// main.rs: argparsing

mod imgload;

// Parse out all images that are not arguments
fn parse_images(args: &mut Vec<String>) -> Vec<String> {
    let mut imgs: Vec<String> = Vec::new();
    let mut rems: Vec<usize> = Vec::new();
    let mut hit_double_hyphen = false;

    // get all the images
    for (x, arg) in args.iter().enumerate() {
        if hit_double_hyphen || arg.chars().next().unwrap() != '-' {
            imgs.push(arg.to_string());
            rems.push(x);
        }
        if arg == "--" {
            hit_double_hyphen = true;
        }
    }

    // remove the parsed images from the arg parser
    rems.iter().fold(0, |accum, x| {
        args.remove(x - accum);
        accum + 1
    });
    return imgs;
}

fn main() {
    // parse the program args
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // skip the program name
    let imgs: Vec<String> = parse_images(&mut args); // remove imgs from args
    let size = imgload::find_largest_resolution(&imgs);
    println!("{:?}", size);
}
