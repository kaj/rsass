use super::{Error, VERSION};
use yaml_rust::{Yaml, YamlLoader};

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
            should_skip: {
                if let Some(skip) = skip_ended(options)? {
                    Some(skip)
                } else {
                    skip_unstarted(options)?
                }
            },
        })
    }
}

fn skip_ended(options: &Yaml) -> Result<Option<String>, Error> {
    if let Some(end) = options[":end_version"].as_str() {
        if end.parse::<f32>()? <= VERSION {
            Ok(Some(format!("end_version is {}", end)))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn skip_unstarted(options: &Yaml) -> Result<Option<String>, Error> {
    if let Some(start) = options[":start_version"].as_str() {
        if start.parse::<f32>()? <= VERSION {
            Ok(None)
        } else {
            Ok(Some(format!("start_version is {}", start)))
        }
    } else {
        Ok(None)
    }
}
