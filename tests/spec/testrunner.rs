use rsass::input::{
    Context, FsLoader, LoadError, Loader, SourceFile, SourceName,
};
use rsass::output::Format;
use rsass::Error;
use std::collections::BTreeMap;
use std::io::{Cursor, Read};
use std::sync::Arc;

pub fn runner() -> TestRunner {
    TestRunner::new()
}

#[derive(Clone)]
pub struct TestRunner {
    format: Format,
    loader: TestLoader,
}

#[derive(Clone, Debug)]
struct TestLoader {
    parent: Arc<FsLoader>,
    mock: BTreeMap<String, String>,
    cwd: String,
}

impl TestLoader {
    fn new() -> Self {
        let mut parent = FsLoader::for_cwd();
        parent.push_path("tests/spec".as_ref());
        TestLoader {
            parent: Arc::new(parent),
            mock: BTreeMap::new(),
            cwd: String::new(),
        }
    }
    fn mock_file(&mut self, name: &str, content: &str) {
        self.mock.insert(url_join(&self.cwd, name), content.into());
    }
    #[allow(unused)] // only used while building tests
    fn has_mock_files(&self) -> bool {
        !self.mock.is_empty()
    }
    fn with_cwd(&mut self, cwd: &str) {
        self.cwd = url_join(&self.cwd, cwd);
    }
}

fn url_join(p: &str, c: &str) -> String {
    if p.is_empty() {
        c.to_string()
    } else if c.is_empty() {
        p.to_string()
    } else if p.ends_with('/') {
        format!("{}{}", p, c)
    } else {
        format!("{}/{}", p, c)
    }
}

impl Loader for TestLoader {
    type File = Cursor<Vec<u8>>;

    fn find_file(&self, name: &str) -> Result<Option<Self::File>, LoadError> {
        let mut cwd = self.cwd.trim_end_matches('/');
        let mut lname = name;
        while let Some(name) = lname.strip_prefix("../") {
            cwd = cwd.rfind('/').map(|p| &self.cwd[..p]).unwrap_or("");
            lname = name;
        }
        let tname = url_join(cwd, lname);

        if let Some(data) = self.mock.get(&tname) {
            return Ok(Some(Cursor::new(data.as_bytes().to_vec())));
        }

        Ok(self.parent.find_file(name)?.map(|mut file| {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            Cursor::new(buf)
        }))
    }
}

impl TestRunner {
    pub fn new() -> TestRunner {
        TestRunner {
            format: Default::default(),
            loader: TestLoader::new(),
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
        self.loader.mock_file(name, content);
        self
    }
    #[allow(unused)] // only used while building tests
    pub fn has_files(&self) -> bool {
        self.loader.has_mock_files()
    }
    pub fn with_cwd(mut self, cwd: &str) -> Self {
        self.loader.with_cwd(cwd);
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
        let name = SourceName::root("input.scss");
        let file = SourceFile::read(&mut input.as_bytes(), name)?;
        Context::for_loader(self.loader.clone())
            .with_format(self.format)
            .transform(file)
    }
}
