// main.rs: argparsing

mod imgload;
mod deviant;

fn main() {
    // parse the program args
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // skip the program name
    let imgs: Vec<String> = parse_images(&mut args); // remove imgs from args
    let args = arg_parse(args);
    if args.output == None {
        panic!("no file to output to specified")
    }

    let (size, poss_largest) = imgload::find_largest_resolution(&imgs, args.verbose.clone());
    if size.0 == 0 || size.1 == 0 {
        panic!("all the images input are invalid!");
    }
    if let Ok(largest_image) = poss_largest {
        let image = deviant::least_average_arr(&imgs, &size, &largest_image, args.verbose.clone());
        imgload::write_image(size, image, args.output.unwrap(), args.verbose.clone()).unwrap();
    }
}

// Parse out all images that are not arguments
fn parse_images(args: &mut Vec<String>) -> Vec<String> {
    let mut imgs: Vec<String> = Vec::new();
    let mut rems: Vec<usize> = Vec::new();
    let mut hit_double_hyphen = false;

    let skip_list = arg_list();
    let mut skip_amt = 0;

    // get all the images and parse out the args
    for (x, arg) in args.iter().enumerate() {
        if skip_amt > 0 {
            skip_amt -= 1;
            continue;
        }

        if hit_double_hyphen || arg.chars().next().unwrap() != '-' {
            imgs.push(arg.to_string());
            rems.push(x);
        } else if arg.chars().next().unwrap() == '-' {
            if arg == "--" {
                hit_double_hyphen = true;
                continue;
            }
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
        args.remove(x - accum);
        accum + 1
    });
    return imgs;
}

// argument struct
struct ParsedArgs {
    output: Option<String>,
    verbose: bool,
}

// build arg list
fn arg_list() -> Vec<(&'static str, u8)> {
    let mut ret = Vec::new();
    ret.push(("-v", 0));
    ret.push(("--verbose", 0));
    ret.push(("-o", 1));
    ret.push(("--output", 1));
    //ret.push(());
    ret
}

// parse out args
fn arg_parse(parsed: Vec<String>) -> ParsedArgs {
    let mut args = ParsedArgs {
        output: None,
        verbose: false,
    };

    for pos in 0..parsed.len() {
        let tester = parsed[pos].as_str();
        match tester.chars().next().unwrap() {
            '-' =>
                match tester {
                    "-o" | "--output" => args.output = Some(parsed[pos + 1].clone()),
                    "-v" | "--verbose" => args.verbose = true,
                    "--" => (),
                    _ => panic!("invalid arg parsed"),
                },
            _ => ()
        }
    }
    args
}

