use rsass::output::Format;
use rsass::{parse_scss_file, Error, FileContext, FsFileContext, ScopeRef};
use std::collections::BTreeMap;

pub fn runner() -> TestRunner {
    TestRunner::new()
}

#[derive(Clone)]
pub struct TestRunner {
    format: Format,
    file_context: TestFileContext,
}

#[derive(Clone, Debug)]
struct TestFileContext {
    parent: FsFileContext,
    mock: BTreeMap<String, String>,
}

impl TestFileContext {
    fn new() -> Self {
        let mut parent = FsFileContext::new();
        parent.push_path("tests/spec".as_ref());
        TestFileContext {
            parent,
            mock: BTreeMap::new(),
        }
    }
    fn mock_file(&mut self, name: &str, content: &str) {
        self.mock.insert(name.into(), content.into());
    }
}

use std::io::Cursor;
use std::io::Read;

impl FileContext for TestFileContext {
    type File = Cursor<Vec<u8>>;

    fn find_file(
        &self,
        name: &str,
    ) -> Result<Option<(Self, String, Self::File)>, Error> {
        for name in vec![
            name.to_string(),
            format!("{}.scss", name),
            format!("_{}.scss", name),
            format!("{}/index.scss", name),
            format!("{}/_index.scss", name),
        ] {
            if let Some(data) = self.mock.get(&name) {
                //let path = full.display().to_string();
                /*return match Self::File::open(full) {
                        Ok(file) => Ok(Some((c, path, file))),
                        Err(e) => Err(Error::Input(path, e)),
                };*/
                return Ok(Some((
                    self.clone(),
                    name,
                    Cursor::new(data.as_bytes().to_vec()),
                )));
            }
        }
        Ok(self.parent.find_file(name)?.map(|(ctx, name, mut file)| {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            (
                TestFileContext {
                    parent: ctx,
                    mock: BTreeMap::new(),
                },
                name,
                Cursor::new(buf),
            )
        }))
    }
}

impl TestRunner {
    pub fn new() -> TestRunner {
        TestRunner {
            format: Default::default(),
            file_context: TestFileContext::new(),
        }
    }
    pub fn set_precision(mut self, precision: usize) -> Self {
        self.format = Format {
            precision,
            ..self.format
        };
        self
    }
    pub fn mock_file(mut self, name: &str, content: &str) -> Self {
        self.file_context.mock_file(name, content);
        self
    }
    #[allow(unused)] // only used while executing tests
    pub fn ok(&self, input: &str) -> String {
        match self.rsass(input) {
            Ok(css) => String::from_utf8(css).unwrap().replace("\n\n", "\n"),
            Err(err) => panic!("Unexpected error:\n{}\n", err),
        }
    }
    #[allow(unused)] // only used while executing tests
    pub fn err(&self, input: &str) -> String {
        match self.rsass(input) {
            Ok(css) => panic!(
                "Unexpected result:\n{}\n",
                String::from_utf8(css).unwrap()
            ),
            Err(err) => err.to_string(),
        }
    }
    #[allow(unused)] // only used while building tests
    pub fn run(&self, input: &str) -> Result<String, String> {
        self.rsass(input)
            .map(|css| String::from_utf8(css).unwrap().replace("\n\n", "\n"))
            .map_err(|err| err.to_string())
    }

    fn rsass(&self, input: &str) -> Result<Vec<u8>, Error> {
        let items = parse_scss_file(&mut input.as_bytes(), "input.scss")?;
        self.format.write_root(
            &items,
            ScopeRef::new_global(self.format),
            &self.file_context,
        )
    }
}
