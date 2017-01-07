extern crate clap;
extern crate rsass;

use clap::{App, Arg, ArgMatches};
use rsass::compile_scss;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

fn main() {
    let args = App::new("rsass")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Rasmus Kaj <rasmus@krats.se>")
        .about("Transform sass to css.")
        .arg(Arg::with_name("PRECISION")
            .long("precision")
            .takes_value(true)
            .help("Ignored"))
        .arg(Arg::with_name("T")
            .short("t")
            .takes_value(true)
            .help("Ignored"))
        .arg(Arg::with_name("INPUT")
            .required(true)
            .multiple(true)
            .help("Sass file to translate"))
        .after_help("At least one INPUT file is required.")
        .get_matches();

    match run(args) {
        Ok(()) => (),
        Err(err) => {
            writeln!(&mut std::io::stderr(), "Error: {}!", err).unwrap();
            exit(1);
        }
    }
}

fn run(args: ArgMatches) -> Result<(), String> {
    if let Some(inputs) = args.values_of("INPUT") {
        for name in inputs {
            let mut source = try!(File::open(&name)
                .map_err(|e| format!("Read {}: {}", name, e)));
            let mut data = vec![];
            try!(source.read_to_end(&mut data).map_err(|e| format!("{}", e)));
            let out = std::io::stdout();
            try!(out.lock()
                .write_all(&try!(compile_scss(&data)))
                .map_err(|e| format!("{}", e)));
        }
    }
    Ok(())
}
