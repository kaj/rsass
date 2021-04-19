//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1804/variable.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: 2px;\
             \n$bar: 5in;\
             \n\
             \nfoo {\
             \n  bar: #{($foo*$bar)};\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: 10px*in isn\'t a valid CSS value.\
         \n  ,\
         \n5 |   bar: #{($foo*$bar)};\
         \n  |          ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 5:10  root stylesheet\
         \n",
    );
}
