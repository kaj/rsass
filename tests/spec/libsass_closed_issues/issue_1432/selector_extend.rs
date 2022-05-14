//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1432/selector-extend.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector-extend")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@debug(selector-extend(\".a .b\", \".b\", null));\n"
        ),
        "Error: $extender: null is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | @debug(selector-extend(\".a .b\", \".b\", null));\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
    );
}
