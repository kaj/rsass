//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_579.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_579")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$map: (\
             \n  foo: fump,\
             \n  bar: bump,\
             \n);\n\
             \n@mixin vararg-test($foo, $bar) {\
             \n  foo: $foo;\
             \n  bar: $bar;\
             \n}\n\
             \n.test {\
             \n  @include vararg-test($map...);\
             \n}\n"),
        ".test {\
         \n  foo: fump;\
         \n  bar: bump;\
         \n}\n"
    );
}
