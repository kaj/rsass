//! Tests auto-converted from "sass-spec/spec/css/custom_properties/script.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("script")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".script {\
             \n  // All script values except interpolation are interpreted statically.\
             \n  --variable: $variable;\
             \n  --operator: 1 + 1;\
             \n  --function: red(#ffffff);\
             \n  --list: (a b c);\
             \n  --map: (a: b, c: d);\
             \n}\n"
        ),
        ".script {\
         \n  --variable: $variable;\
         \n  --operator: 1 + 1;\
         \n  --function: red(#ffffff);\
         \n  --list: (a b c);\
         \n  --map: (a: b, c: d);\
         \n}\n"
    );
}
