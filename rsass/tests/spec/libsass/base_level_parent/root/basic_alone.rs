//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/basic-alone.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic-alone")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("& {\
             \n  foo {\
             \n    bar: baz;\
             \n  }\
             \n}\n"),
        "& foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
