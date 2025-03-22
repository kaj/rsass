//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_813.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_813")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function foo($one, $two) {\
             \n  @return $one + $two;\
             \n}\n\
             \n$nums: 1px 2px;\n\
             \n.foo {\
             \n  left: foo($nums...);\
             \n  bottom: $nums 3px;\
             \n}\n"),
        ".foo {\
         \n  left: 3px;\
         \n  bottom: 1px 2px 3px;\
         \n}\n"
    );
}
