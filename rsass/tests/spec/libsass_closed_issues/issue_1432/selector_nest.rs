//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1432/selector-nest.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector-nest")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:selector\";\
             \n@debug(selector.nest(\"foo\", null));\n"
        ),
        "Error: null is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n2 | @debug(selector.nest(\"foo\", null));\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet",
    );
}
