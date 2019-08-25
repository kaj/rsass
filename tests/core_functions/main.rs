//! Tests auto-converted from "sass-spec/spec/core_functions"
//! version cb1ab541, 2019-08-16 17:59:57 -0700.
//! See <https://github.com/sass/sass-spec> for source material.\n
use rsass::{compile_scss, OutputStyle};

mod color;

mod list;

mod map;

mod math;

mod meta;

mod selector;

mod string;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
