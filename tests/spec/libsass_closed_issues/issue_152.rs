//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_152.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: 10;\
             \n$bar: 10%;\
             \n\
             \nfoo {\
             \n  a: #{10}% 100%;\
             \n  a: #{10} % 100%;\
             \n  a: #{10} %100%;\
             \n  a: 10% 100%;\
             \n  a: 10 % 100%;\
             \n  a: 10 %100%;\
             \n  a: $foo 100%;\
             \n  a: $foo % 100%;\
             \n  a: $foo %100%;\
             \n  a: $bar 100%;\
             \n  a: $bar % 100%;\
             \n  a: $bar %100%;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Undefined operation \"10 % 100%\".\
         \n  ,\
         \n5 |   a: #{10}% 100%;\
         \n  |      ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 5:6  root stylesheet\
         \n",
    );
}
