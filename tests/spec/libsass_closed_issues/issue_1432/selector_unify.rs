//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1432/selector-unify.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@debug(selector-unify(\"foo\", null));\
             \n"
        ).unwrap_err(),
        "Error: $selector2: null is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | @debug(selector-unify(\"foo\", null));\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
    );
}
