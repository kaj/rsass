//! Tests auto-converted from "sass-spec/spec/css/custom_properties/without_semicolon.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("without_semicolon")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            ".simple-value {\
             \n  // A custom property at the end of a style rule doesn\'t need a semicolon.\
             \n  --without-semicolon: value\
             \n}\n\
             \n.bracketed-value {\
             \n  --without-semicolon: {\
             \n    a: b\
             \n  }\
             \n}\n"
        ),
        ".simple-value {\
         \n  --without-semicolon: value ;\
         \n}\
         \n.bracketed-value {\
         \n  --without-semicolon: {\
         \n    a: b\
         \n  } ;\
         \n}\n"
    );
}
