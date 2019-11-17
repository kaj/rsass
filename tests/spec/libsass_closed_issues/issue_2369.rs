//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2369.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@a(#{url(\\#{})}\
             \n"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n1 | @a(#{url(\\#{})}\
         \n  |           ^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet\
         \n",
    );
}
