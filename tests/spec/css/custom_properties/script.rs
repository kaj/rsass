//! Tests auto-converted from "sass-spec/spec/css/custom_properties/script.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".script {\
            \n  // All script values except interpolation are interpreted statically.\
            \n  --variable: $variable;\
            \n  --operator: 1 + 1;\
            \n  --function: red(#ffffff);\
            \n  --list: (a b c);\
            \n  --map: (a: b, c: d);\
            \n}\
            \n"
        )
        .unwrap(),
        ".script {\
        \n  --variable: $variable;\
        \n  --operator: 1 + 1;\
        \n  --function: red(#ffffff);\
        \n  --list: (a b c);\
        \n  --map: (a: b, c: d);\
        \n}\
        \n"
    );
}
