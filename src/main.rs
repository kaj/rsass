#[cfg(feature = "commandline")]
extern crate clap;
extern crate rsass;

#[cfg(feature = "commandline")]
use clap::{App, Arg, ArgMatches};
#[cfg(feature = "commandline")]
use rsass::{compile_scss_file, Error, OutputStyle};
#[cfg(feature = "commandline")]
use std::io::{stderr, stdout, Write};
use std::process::exit;

#[cfg(not(feature = "commandline"))]
fn main() {
    // I would like to not build a command-line at all when the
    // commandline feature is not given, but I haven't figured out a
    // way to do that in cargo, so instead I make a simple commandline
    // which only gives this error message.
    eprintln!("Error: rsass is built without the commandline feautre");
    exit(1);
}

#[cfg(feature = "commandline")]
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
        )
        .arg(
            Arg::with_name("STYLE")
                .short("t")
                .long("style")
                .takes_value(true)
                .help("Output style. Can be compact (default) or compressed."),
        )
        .arg(
            Arg::with_name("INPUT")
                .required(true)
                .multiple(true)
                .help("Sass file(s) to translate"),
        )
        .after_help("At least one INPUT file is required.")
        .get_matches();

    match run(&args) {
        Ok(()) => (),
        Err(err) => {
            writeln!(&mut stderr(), "Error: {}!", err).unwrap();
            exit(1);
        }
    }
}

#[cfg(feature = "commandline")]
fn run(args: &ArgMatches) -> Result<(), Error> {
    let style = if args.value_of("STYLE") == Some("compressed") {
        OutputStyle::Compressed
    } else {
        OutputStyle::Normal
    };
    if let Some(inputs) = args.values_of("INPUT") {
        for name in inputs {
            let result = compile_scss_file(name.as_ref(), style)?;
            let out = stdout();
            out.lock().write_all(&result)?;
        }
    }
    Ok(())
}
