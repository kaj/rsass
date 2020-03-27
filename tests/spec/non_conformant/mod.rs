//! Tests auto-converted from "sass-spec/spec/non_conformant"
#[allow(unused)]
use super::rsass;

mod basic;

mod colors;

mod errors;

mod extend_tests;

// From "sass-spec/spec/non_conformant/media_import.hrx"
#[test]
fn media_import() {
    assert_eq!(
        rsass("@import \"./fonts.sass\" all;").unwrap(),
        "@import \"./fonts.sass\" all;\
        \n"
    );
}

mod misc;

mod mixin;

mod nesting;

mod operations;

mod parser;

mod sass;

mod sass_4_0;

mod scope;

mod scss;

mod scss_tests;

mod selectors;

// From "sass-spec/spec/non_conformant/variables.hrx"
mod variables {
    #[allow(unused)]
    use super::rsass;
    mod global {
        #[allow(unused)]
        use super::rsass;
        mod first_declaration {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn nested() {
                assert_eq!(
                    rsass(
                        "x {$var: value !global}\
            \na {b: $var}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: value;\
        \n}\
        \n"
                );
            }
            #[test]
            fn top_level() {
                assert_eq!(
                    rsass(
                        "$var: value !global;\
            \na {b: $var}\
            \n"
                    )
                    .unwrap(),
                    "a {\
        \n  b: value;\
        \n}\
        \n"
                );
            }
        }
    }
}
