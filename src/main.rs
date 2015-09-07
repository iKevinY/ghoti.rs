extern crate getopts;

use getopts::Options;

use std::env;
use std::fs::File;
use std::path::Path;


fn print_usage(program: &str, opts: Options) {
    let usage = format!("Usage: {} [options] <filename>", program);
    print!("{}", opts.usage(&usage));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print version info");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("version") {
        println!("ghoti {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if matches.opt_present("help") || matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    }

    let filename = matches.free[0].clone();

    match File::open(Path::new(&filename)) {
        Ok(_) => println!("Hello, {}!", filename),
        Err(_) => panic!("Couldn't open {}.", filename),
    };
}
