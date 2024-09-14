//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/media/script_features.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("script_features")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:list\";\
             \n$foo: 3;\
             \n$bar: 4;\
             \n// Media features are special-cased to allow raw script without interpolation.\
             \n@media only screen and (max-width: $foo) and (min-width: $bar) {x {y: z}}\n\
             \n// Not just variables, but full script\
             \n$vals: 1 2 3;\
             \n@media screen and (max-width: 1 + 2) and (min-width: 5 + 6 + list.nth($vals, 2)) {x {y: z}}\n\n"
        ),
        "@media only screen and (max-width: 3) and (min-width: 4) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\
         \n@media screen and (max-width: 3) and (min-width: 13) {\
         \n  x {\
         \n    y: z;\
         \n  }\
         \n}\n"
    );
}
