// args.rs: arg parsing

use std::sync::Arc;

// argument struct
pub struct ParsedArgs {
    pub output: Option<String>,
    pub verbose: bool,
    pub largest: Option<String>,
}


// parse out args
pub fn arg_parse(parsed: Vec<String>) -> Arc<ParsedArgs> {
    let mut args = ParsedArgs {
        output: None,
        verbose: false,
        largest: None,
    };

    // update the struct with the args
    for pos in 0..parsed.len() {
        let tester = parsed[pos].as_str();
        match tester.chars().next().unwrap() {
            '-' =>
                match tester {
                    "-o" | "--output" => args.output = Some(parsed[pos + 1].clone()),
                    "-v" | "--verbose" => args.verbose = true,
                    "-b" | "--base" => args.largest = Some(parsed[pos + 1].clone()),
                    "--" => (),
                    _ => panic!("invalid arg parsed"),
                },
            _ => ()
        }
    }
    Arc::new(args)
}

// build arg skip list
pub fn arg_skip_list() -> Vec<(&'static str, u8)> {
    let mut ret = Vec::new();
    ret.push(("-v", 0));
    ret.push(("--verbose", 0));
    ret.push(("-o", 1));
    ret.push(("--output", 1));
    ret.push(("-b", 1));
    ret.push(("--base", 1));
    ret
}
