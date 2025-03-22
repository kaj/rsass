//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_556.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_556")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$test: (\
             \n  one: 1,\
             \n  two: 2,\
             \n);\n\
             \n$expect: (\
             \n  two: 2,\
             \n  one: 1,\
             \n);\n\
             \n.test {\
             \n  equal: $test == $expect;\
             \n}\n"),
        ".test {\
         \n  equal: true;\
         \n}\n"
    );
}
