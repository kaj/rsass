use rsass::output::Format;
use rsass::{parse_scss_file, Error, FsFileContext, ScopeRef};

struct TestRunner {
    format: Format,
    file_context: FsFileContext,
}

impl TestRunner {
    fn new() -> TestRunner {
        let mut file_context = FsFileContext::new();
        file_context.push_path("tests/spec".as_ref());
        TestRunner {
            format: Default::default(),
            file_context,
        }
    }
    fn set_precision(mut self, precision: usize) -> Self {
        self.format = Format {
            precision,
            ..self.format
        };
        self
    }
    fn rsass(&self, input: &str) -> Result<Vec<u8>, Error> {
        let items = parse_scss_file(&mut input.as_bytes(), "input.scss")?;
        self.format.write_root(
            &items,
            ScopeRef::new_global(self.format),
            &self.file_context,
        )
    }
    fn ok(&self, input: &str) -> String {
        match self.rsass(input) {
            Ok(css) => {
                String::from_utf8(css)
                    .unwrap()
                    .replace("\n\n", "\n")
            }
            Err(err) => panic!("Unexpected error:\n{}\n", err),
        }
    }
    fn err(&self, input: &str) -> String {
        match self.rsass(input) {
            Ok(css) => panic!("Unexpected result:\n{}\n", String::from_utf8(css).unwrap()),
            Err(err) => err.to_string(),
        }
    }
}
