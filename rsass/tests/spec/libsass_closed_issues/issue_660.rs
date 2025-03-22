//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_660.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_660")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$foo: true;\n\
             \ndiv {\
             \n  blah: $foo;\
             \n}\n\
             \ndiv {\
             \n  blah: not $foo;\
             \n}\n\
             \ndiv {\
             \n  blah: not ($foo);\
             \n}\n\
             \ndiv {\
             \n  blah: not (true);\
             \n}\n\n"),
        "div {\
         \n  blah: true;\
         \n}\
         \ndiv {\
         \n  blah: false;\
         \n}\
         \ndiv {\
         \n  blah: false;\
         \n}\
         \ndiv {\
         \n  blah: false;\
         \n}\n"
    );
}
