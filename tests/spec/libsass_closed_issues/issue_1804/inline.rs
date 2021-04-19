//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1804/inline.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
             \n  bar: #{(2px*5in)};\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: 10px*in isn\'t a valid CSS value.\
         \n  ,\
         \n2 |   bar: #{(2px*5in)};\
         \n  |          ^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:10  root stylesheet\
         \n",
    );
}
