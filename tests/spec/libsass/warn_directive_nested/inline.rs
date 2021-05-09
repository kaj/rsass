//! Tests auto-converted from "sass-spec/spec/libsass/warn-directive-nested/inline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  b: {\
             \n    @warn test;\
             \n    c: d;\
             \n  }\
             \n}\n"),
        "a {\
         \n  b-c: d;\
         \n}\n"
    );
}
