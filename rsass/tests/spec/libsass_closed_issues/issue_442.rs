//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_442.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_442")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$lhs: (100/10)#{rem};\
             \n$rhs: 10rem;\n\
             \nfoo {\
             \n  a: $lhs;\
             \n  a: $rhs;\
             \n  a: $lhs == $rhs;\
             \n}\n"),
        "foo {\
         \n  a: 10 rem;\
         \n  a: 10rem;\
         \n  a: false;\
         \n}\n"
    );
}
