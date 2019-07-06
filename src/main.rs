use clap::{value_t, App, Arg, ArgMatches};
use rsass::{
    parse_scss_file, set_precision, Error, FileContext, GlobalScope,
    OutputStyle,
};
use std::io::{stdout, Write};
use std::process::exit;

fn main() {
    let args = App::new("rsass")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Rasmus Kaj <rasmus@krats.se>")
        .about("Transform sass to css.")
        .version_short("v")
        .arg(
            Arg::with_name("PRECISION")
                .long("precision")
                .takes_value(true)
                .default_value("5")
                .validator(|p| {
                    p.parse::<usize>()
                        .map(|_| ())
                        .map_err(|e| format!("Expected integer: {}", e))
                })
                .help(
                    "How many digits of precision to use when \
                     outputting decimal numbers.",
                ),
        )
        .arg(
            Arg::with_name("STYLE")
                .short("t")
                .long("style")
                .takes_value(true)
                .possible_values(&OutputStyle::variants())
                .default_value(&OutputStyle::variants()[1])
                .case_insensitive(true)
                .help("Output style"),
        )
        .arg(
            Arg::with_name("INCLUDE_PATH")
                .short("I")
                .takes_value(true)
                .help("Where to search for included resources."),
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
            eprintln!("{}", err);
            exit(1);
        }
    }
}

fn run(args: &ArgMatches) -> Result<(), Error> {
    let style = value_t!(args, "STYLE", OutputStyle)?;
    set_precision(value_t!(args, "PRECISION", usize)?);
    if let Some(inputs) = args.values_of("INPUT") {
        for name in inputs {
            let mut file_context = FileContext::new();
            if let Some(include_path) = args.value_of("INCLUDE_PATH") {
                file_context.push_path(include_path.as_ref());
            }
            let (sub_context, file) = file_context.file(name.as_ref());
            let items = parse_scss_file(&file)?;
            let result = style.write_root(
                &items,
                &mut GlobalScope::new(),
                &sub_context,
            )?;
            let out = stdout();
            out.lock().write_all(&result)?;
        }
    }
    Ok(())
}
