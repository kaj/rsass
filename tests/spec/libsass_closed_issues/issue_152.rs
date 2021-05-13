//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_152.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "$foo: 10;\
             \n$bar: 10%;\n\
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
             \n}\n"
        ),
        "Error: Undefined operation \"10 % 100%\".\
         \n  ,\
         \n5 |   a: #{10}% 100%;\
         \n  |      ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 5:6  root stylesheet",
    );
}
