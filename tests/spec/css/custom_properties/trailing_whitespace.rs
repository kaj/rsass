//! Tests auto-converted from "sass-spec/spec/css/custom_properties/trailing_whitespace.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".trailing-whitespace {\
             \n  --space: value ;\
             \n  --tab: value\t;\
             \n  --newline: value\
             \n;\
             \n  --before-closing-brace: value\
             \n}\n"),
        ".trailing-whitespace {\
         \n  --space: value ;\
         \n  --tab: value\t;\
         \n  --newline: value ;\
         \n  --before-closing-brace: value ;\
         \n}\n"
    );
}
