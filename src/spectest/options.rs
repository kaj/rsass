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
        match &YamlLoader::load_from_str(&options)?[..] {
            [] => Err(Error(format!("Found zero-doc options {:?}", options))),
            [options] => {
                //eprintln!("Found options: {:?}", options);
                Ok(Options {
                    precision: options[":precision"].as_i64(),
                    // Target version no longer used by sass-spec,
                    // and no other reasons to skip implemented here.
                    should_skip: None,
                })
            }
            multiple => Err(Error(format!(
                "Found multiple-doc options {:?}",
                multiple
            ))),
        }
    }
}
