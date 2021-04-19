//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2371.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "#{a==b}\
             \n"
        )
        .unwrap_err(),
        "Error: expected \"{\".\
         \n  ,\
         \n1 | #{a==b}\
         \n  |        ^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet\
         \n",
    );
}
