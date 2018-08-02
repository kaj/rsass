#[macro_use]
extern crate clap;
extern crate rsass;

use clap::{App, Arg, ArgMatches};
use rsass::{compile_scss_file, Error, OutputStyle};
use std::io::{stdout, Write};
use std::process::exit;

fn main() {
    let args = App::new("rsass")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Rasmus Kaj <rasmus@krats.se>")
        .about("Transform sass to css.")
        .arg(
            Arg::with_name("PRECISION")
                .long("precision")
                .takes_value(true)
                .help("Ignored"),
        ).arg(
            Arg::with_name("STYLE")
                .short("t")
                .long("style")
                .takes_value(true)
                .possible_values(&OutputStyle::variants())
                .default_value(&OutputStyle::variants()[1])
                .case_insensitive(true)
                .help("Output style"),
        ).arg(
            Arg::with_name("INPUT")
                .required(true)
                .multiple(true)
                .help("Sass file(s) to translate"),
        ).after_help("At least one INPUT file is required.")
        .get_matches();

    match run(&args) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("Error: {}!", err);
            exit(1);
        }
    }
}

fn run(args: &ArgMatches) -> Result<(), Error> {
    let style = value_t!(args, "STYLE", OutputStyle)?;
    if let Some(inputs) = args.values_of("INPUT") {
        for name in inputs {
            let result = compile_scss_file(name.as_ref(), style)?;
            let out = stdout();
            out.lock().write_all(&result)?;
        }
    }
    Ok(())
}
