//! Tests auto-converted from "sass-spec/spec/libsass/debug-directive-nested/inline.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a {\
             \n  b: {\
             \n    @debug test;\
             \n    c: d;\
             \n  }\
             \n}\n"),
        "a {\
         \n  b-c: d;\
         \n}\n"
    );
}
