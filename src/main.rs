extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Recho")
        .version("0.1.0")
        .author("Christian Roese <croese@gmail.com>")
        .about("Write arguments to the standard output")
        .arg(
            Arg::with_name("n")
                .short("n")
                .help("do not append a newline"),
        )
        .arg(Arg::with_name("args").multiple(true).takes_value(true))
        .get_matches();
    let suppress_newlines = matches.is_present("n");
    let args = match matches.values_of("args") {
        None => Vec::new(),
        Some(a) => a.collect(),
    };
    let joined = args.join(" ");
    let terminator = if suppress_newlines { "" } else { "\n" };
    print!("{}{}", joined, terminator);
}
