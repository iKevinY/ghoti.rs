extern crate getopts;

use getopts::Options;

use std::env;
use std::fs::File;
use std::path::Path;



fn print_usage(opts: Options) {
    let usage = "Usage: ghoti [options] <filename>";
    print!("{}", opts.usage(usage));
}

fn main() {
    // Set up CLI options
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print version info");

    // Ignore the first element of env::args() (program name)
    let args: Vec<String> = env::args().skip(1).collect();
    let matches = opts.parse(&args).unwrap();

    // Iterate through CLI flags
    if matches.opt_present("version") {
        println!("ghoti {}", env!("CARGO_PKG_VERSION"));
        return;
    } else if matches.opt_present("help") || matches.free.is_empty() {
        print_usage(opts);
        return;
    }

    let filename = matches.free[0].clone();

    match File::open(Path::new(&filename)) {
        Ok(_) => println!("Hello, {}!", filename),
        Err(_) => panic!("Couldn't open {}.", filename),
    };
}
