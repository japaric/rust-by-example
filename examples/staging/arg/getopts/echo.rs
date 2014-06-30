extern crate getopts;
extern crate libc;

use std::os;
use std::io::{print, println};
use std::io::stdio::flush;

static VERSION: &'static str = "1.0.0";

fn main() {
    let args = os::args();
    let program = args.get(0).clone();

    // Set possible flags.
    // The first argument to `optflag` is the short flag name.
    // The second argument is the long flag name.
    // The third argument is the help text.
    let opts = [
        getopts::optflag("n", "", "do not output the trailing newline"),
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("V", "version", "output version information and exit"),
    ];

    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            os::set_exit_status(1);
            return;
            // The exit code is 0 (success) by default.
            // Any exit code other than 0 indicates failure.
        }
    };

    if matches.opt_present("help") {
        //^ We could as well have used the short name.
        println!("echo {:s} - display a line of text", VERSION);
        println!("");
        println!("Usage:");
        println!(" {0:s} [SHORT-OPTION]... [STRING]...", program);
        println!(" {0:s} LONG-OPTION", program);
        println!("");
        println(getopts::usage("Echo the STRING(s) to standard output.", opts)
                .as_slice());
        return;
    }

    if matches.opt_present("version") {
        println!("echo version: {:s}", VERSION);
        return;
    }

    if !matches.free.is_empty() {
        //^ `matches.free` contains all arguments that are not options.
        let string = matches.free.connect(" ");
        print(string.as_slice());
    } 

    if !matches.opt_present("n") {
        println!("")
    } else {
        flush();
    }
}
