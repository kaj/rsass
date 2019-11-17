//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1432/simple-selectors.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@debug(simple-selectors(null));\
             \n"
        ).unwrap_err(),
        "Error: $selector: null is not a valid selector: it must be a string,\
         \na list of strings, or a list of lists of strings.\
         \n  ,\
         \n1 | @debug(simple-selectors(null));\
         \n  |        ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet\
         \n",
    );
}
