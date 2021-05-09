//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2779.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err("@debug(selector-extend(\".a .b\", \"&b\", ndll));\n"),
        "Error: $extendee: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &b\
         \n  | ^^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n1 | @debug(selector-extend(\".a .b\", \"&b\", ndll));\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
    );
}
