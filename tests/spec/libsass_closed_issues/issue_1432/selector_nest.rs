//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1432/selector-nest.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err("@debug(selector-nest(\"foo\", null));\n"),
        "Error: null is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | @debug(selector-nest(\"foo\", null));\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
    );
}
