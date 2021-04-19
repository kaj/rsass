//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2779.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@debug(selector-extend(\".a .b\", \"&b\", ndll));\
             \n"
        )
        .unwrap_err(),
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
         \n  input.scss 1:8  root stylesheet\
         \n",
    );
}
