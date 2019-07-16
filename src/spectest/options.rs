use super::Error;
use yaml_rust::YamlLoader;

#[derive(Default)]
pub struct Options {
    pub precision: Option<i64>,
    /// None for tests that should work, or Some(reason to skip).
    pub should_skip: Option<String>,
}

impl Options {
    pub fn parse(options: &str) -> Result<Options, Error> {
        let options = YamlLoader::load_from_str(&options)?;
        if options.len() > 1 {
            Err(Error(format!("Found multiple-doc options {:?}", options)))?;
        }
        if options.len() == 0 {
            Err(Error(format!("Found zero-doc options {:?}", options)))?;
        }
        let options = &options[0];
        eprintln!("Found options: {:?}", options);
        Ok(Options {
            precision: options[":precision"].as_i64(),
            // Target version no longer used by sass-spec,
            // and no other reasons to skip implemented here.
            should_skip: None,
        })
    }
}
