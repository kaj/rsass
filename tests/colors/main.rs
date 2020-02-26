//! Tests auto-converted from "sass-spec/spec/colors"
//! version e9e219bdf, 2019-12-19 17:12:28 -0800.
//! See <https://github.com/sass/sass-spec> for source material.\n
use rsass::{compile_scss, OutputFormat};

// From "sass-spec/spec/colors/basic.hrx"
#[test]
fn basic() {
    assert_eq!(
        rsass(
            "p {\
            \n  color: rgb(255, 128, 0);\
            \n  color: red green blue;\
            \n  color: (red) (green) (blue);\
            \n  color: red + hux;\
            \n  color: unquote(\"red\") + green;\
            \n  foo: rgb(200, 150%, 170%);\
            \n}"
        )
        .unwrap(),
        "p {\
        \n  color: #ff8000;\
        \n  color: red green blue;\
        \n  color: red green blue;\
        \n  color: redhux;\
        \n  color: redgreen;\
        \n  foo: #c8ffff;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/colors/change-color.hrx"
#[test]
fn change_color() {
    assert_eq!(
        rsass(
            "p {\
            \n  color: change-color(#102030, $blue: 5);\
            \n  color: change-color(#102030, $alpha: .325);\
            \n  color: change-color(#102030, $red: 120, $blue: 5);\
            \n  color: change-color(hsl(25, 100%, 80%), $lightness: 40%, $alpha: 0.8);\
            \n}"
        )
        .unwrap(),
        "p {\
        \n  color: #102005;\
        \n  color: rgba(16, 32, 48, 0.325);\
        \n  color: #782005;\
        \n  color: rgba(204, 85, 0, 0.8);\
        \n}\
        \n"
    );
}

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputFormat::default())
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
#[allow(unused)]
fn rsass_fmt(format: OutputFormat, input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), format)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
