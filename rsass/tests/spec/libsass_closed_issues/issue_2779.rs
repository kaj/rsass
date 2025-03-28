//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2779.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2779")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \n@debug(selector.extend(\".a .b\", \"&b\", ndll));\n"
        ),
        "Error: $extendee: Parent selectors aren\'t allowed here.\
         \n  ,\
         \n1 | &b\
         \n  | ^^\
         \n  \'\
         \n  - 1:1  root stylesheet\
         \n  ,\
         \n2 | @debug(selector.extend(\".a .b\", \"&b\", ndll));\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet",
    );
}
