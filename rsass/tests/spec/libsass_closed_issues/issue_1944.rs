//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1944.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1944")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$count: 0;\n\
             \n@function foo() {\
             \n    $count: $count + 1 !global;\
             \n    $selector: (\'.bar\' \'baz\');\
             \n    @return $selector;\
             \n}\n\n\
             \n#{foo()} {\
             \n    count: $count;\
             \n}\n"),
        ".bar baz {\
         \n  count: 1;\
         \n}\n"
    );
}
