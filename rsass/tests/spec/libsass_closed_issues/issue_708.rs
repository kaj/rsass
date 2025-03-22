//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_708.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_708")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foobar($x, $y, $z : 3) {\
             \n  @return $x + $y * 2 + $z\
             \n}\n\
             \n.foobar {\
             \n  content: foobar($y:2, $x:4);\
             \n  content: foobar($y: 2, $x: 4);\
             \n  content: foobar($y : 2, $x : 4);\
             \n  content: foobar($y:2px, $x:4);\
             \n  content: foobar($y: 2px, $x: 4);\
             \n  content: foobar($y : 2px, $x : 4);\
             \n}"),
        ".foobar {\
         \n  content: 11;\
         \n  content: 11;\
         \n  content: 11;\
         \n  content: 11px;\
         \n  content: 11px;\
         \n  content: 11px;\
         \n}\n"
    );
}
