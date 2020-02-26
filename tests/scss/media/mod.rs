//! Tests auto-converted from "sass-spec/spec/scss/media"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/scss/media/interpolated.hrx"
#[test]
fn interpolated() {
    assert_eq!(
        rsass(
            "// You can interpolate into a media type.\
            \n@media bar#{12} {x {y: z}}\
            \n\
            \n// Media queries should be reparsed after interpolation is resolved.\
            \n@media #{\"only screen\"} and\
            \n       #{\"(min-width: 700px)\"} and\
            \n       #{\"(max-width: \"+\"1920px)\"} {\
            \n  x {y: z}\
            \n}\
            \n\
            \n// Queries don\'t have to fully parse before interpolation is resolved.\
            \n@media scr#{\"een, pri\"}nt a#{\"nd (max-width: 300px)\"} {x {y: z}}\
            \n\
            \n\
            \n\
            \n"
        )
        .unwrap(),
        "@media bar12 {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media only screen and (min-width: 700px) and (max-width: 1920px) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media screen, print and (max-width: 300px) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n"
    );
}

mod nesting;

// From "sass-spec/spec/scss/media/script_features.hrx"
#[test]
fn script_features() {
    assert_eq!(
        rsass(
            "$foo: 3;\
            \n$bar: 4;\
            \n// Media features are special-cased to allow raw script without interpolation.\
            \n@media only screen and (max-width: $foo) and (min-width: $bar) {x {y: z}}\
            \n\
            \n// Not just variables, but full script\
            \n$vals: 1 2 3;\
            \n@media screen and (max-width: 1 + 2) and (min-width: 5 + 6 + nth($vals, 2)) {x {y: z}}\
            \n\
            \n"
        )
        .unwrap(),
        "@media only screen and (max-width: 3) and (min-width: 4) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media screen and (max-width: 3) and (min-width: 13) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n"
    );
}
