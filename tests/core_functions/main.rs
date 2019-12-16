//! Tests auto-converted from "sass-spec/spec/core_functions"
//! version 0749da435, 2019-11-26 09:46:59 -0800.
//! See <https://github.com/sass/sass-spec> for source material.\n
use rsass::{compile_scss, OutputStyle};

mod color;

mod list;

mod map;

mod math;

mod meta;

mod modules;

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
