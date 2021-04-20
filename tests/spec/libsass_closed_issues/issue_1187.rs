//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1187.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "$a: \'foo\';\
             \n$b: \'foo\';\
             \n$map: (\
             \n  $a: 1,\
             \n  $b: 2\
             \n);\
             \n\
             \n.foo {\
             \n  content: $a == $b;\
             \n  content: inspect($map);\
             \n}"
        )
        .unwrap_err(),
        "Error: Duplicate key.\
         \n  ,\
         \n4 |   $a: 1,\
         \n  |   == first key\
         \n5 |   $b: 2\
         \n  |   ^^ second key\
         \n  \'\
         \n  input.scss 5:3  root stylesheet",
    );
}
