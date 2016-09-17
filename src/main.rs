extern crate ghoti;
extern crate getopts;

use getopts::Options;
use ghoti::correction;
use std::env;


fn print_usage(opts: Options) {
    let usage = "Usage: ghoti [options] <word>";
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

    let word = matches.free[0].to_lowercase();
    println!("{}", correction(&word));
}
