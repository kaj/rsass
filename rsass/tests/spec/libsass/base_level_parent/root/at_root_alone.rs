//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/root/at-root-alone.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-root-alone")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@at-root {\
             \n  & {\
             \n    foo {\
             \n      bar: baz;\
             \n    }\
             \n  }\
             \n}"),
        "& foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
