extern crate clap;
extern crate rsass;

use clap::{App, Arg, ArgMatches};
use rsass::{OutputStyle, compile_scss_file};
use std::io::Write;
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
        .arg(Arg::with_name("STYLE")
            .short("t")
            .long("style")
            .takes_value(true)
            .help("Output style. Can be compact (default) or compressed."))
        .arg(Arg::with_name("INPUT")
            .required(true)
            .multiple(true)
            .help("Sass file(s) to translate"))
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
    let style = if args.value_of("STYLE") == Some("compressed") {
        OutputStyle::Compressed
    } else {
        OutputStyle::Normal
    };
    if let Some(inputs) = args.values_of("INPUT") {
        for name in inputs {
            let result = try!(compile_scss_file(name.as_ref(), style.clone()));
            let out = std::io::stdout();
            try!(out.lock()
                .write_all(&result)
                .map_err(|e| format!("{}", e)));
        }
    }
    Ok(())
}
