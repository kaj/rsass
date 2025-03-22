//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1333.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1333")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function baz() {\
             \n    @return \'baz\';\
             \n}\n\
             \nfoo {\
             \n    bar: baz()#{\' !important\'};\
             \n    bar: baz() #{\' !important\'};\
             \n}\n\n"),
        "foo {\
         \n  bar: \"baz\"  !important;\
         \n  bar: \"baz\"  !important;\
         \n}\n"
    );
}
