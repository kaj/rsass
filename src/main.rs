use rsass::{
    output::{Format, Style},
    parse_scss_path, Error, FsFileContext, ScopeRef,
};
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;

fn main() {
    match Args::from_args().run() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

#[derive(StructOpt)]
#[structopt(
    about,
    author,
    version_short = "v",
    after_help = "For information about rsass and its current state of \
                  development, please refer to https://github.com/kaj/rsass/ .\
                  \n\n\
                  The sass / scss languate itself is documented at \
                  https://sass-lang.com/ ."
)]
struct Args {
    /// How many digits of precision to use when outputting decimal numbers.
    #[structopt(long, default_value = "5")]
    precision: usize,

    /// How to format output.
    #[structopt(long, short = "t", case_insensitive = true,
                default_value = "expanded",
                possible_values = Style::variants())]
    style: Style,

    /// Where to search for included resources.
    #[structopt(long, short = "I")]
    include_path: Option<PathBuf>,

    /// Sass file(s) to translate
    #[structopt(required = true)]
    input: Vec<PathBuf>,
}

impl Args {
    fn run(self) -> Result<(), Error> {
        let format = Format {
            style: self.style,
            precision: self.precision,
        };
        for name in &self.input {
            let mut file_context = FsFileContext::new();
            if let Some(include_path) = &self.include_path {
                file_context.push_path(include_path.as_ref());
            }
            let (sub_context, path) = file_context.file(name.as_ref());
            let items = parse_scss_path(&path)?;
            let result = format.write_root(
                &items,
                ScopeRef::new_global(format),
                &sub_context,
            )?;
            let out = stdout();
            out.lock().write_all(&result)?;
        }
        Ok(())
    }
}
