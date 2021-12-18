// main.rs

mod imgload;
mod deviant;
mod args;

fn main() {
    // parse the program args
    let mut argstrs: Vec<String> = std::env::args().collect();
    argstrs.remove(0); // skip the program name
    let imgs: Vec<String> = parse_images(&mut argstrs); // remove imgs from args
    let argstruct = args::arg_parse(argstrs);
    if argstruct.output == None {
        panic!("no file to output to specified")
    }

    // find largest resolution for basis image
    let (size, poss_largest) = imgload::find_largest_resolution(&imgs, &argstruct);
    if size.0 == 0 || size.1 == 0 {
        panic!("all the images input are invalid!");
    }

    // create the least average image and write it out
    if let Ok(largest_image) = poss_largest {
        let image = deviant::least_average_img(imgs, size, largest_image, argstruct.clone());
        imgload::write_image(size, image, argstruct.clone()).unwrap();
    }
}

// Parse out all images that are not arguments
fn parse_images(argstrs: &mut Vec<String>) -> Vec<String> {
    let mut imgs: Vec<String> = Vec::new();
    let mut rems: Vec<usize> = Vec::new();
    let mut hit_double_hyphen = false;

    let skip_list = args::arg_skip_list();
    let mut skip_amt = 0;

    // get all the images and parse out the args
    for (x, arg) in argstrs.iter().enumerate() {
        // if arg has been hit earlier, skip its arguments
        if skip_amt > 0 {
            skip_amt -= 1;
            continue;
        }

        // regular image
        if hit_double_hyphen || arg.chars().next().unwrap() != '-' {
            imgs.push(arg.to_string());
            rems.push(x);

        }
        // args
        else if arg.chars().next().unwrap() == '-' {
            // double hyphen does something special
            if arg == "--" {
                hit_double_hyphen = true;
                continue;
            }

            // add amount to skip because iter is immutable
            for (name, skip) in skip_list.iter() {
                if arg == name {
                    skip_amt = *skip;
                    break;
                }
            }
        }
    }

    // remove the parsed images from the arg parser
    rems.iter().fold(0, |accum, x| {
        argstrs.remove(x - accum);
        accum + 1
    });
    return imgs;
}
