//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1187.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1187")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:meta\";\n\
             \n$a: \'foo\';\
             \n$b: \'foo\';\
             \n$map: (\
             \n  $a: 1,\
             \n  $b: 2\
             \n);\n\
             \n.foo {\
             \n  content: $a == $b;\
             \n  content: meta.inspect($map);\
             \n}"
        ),
        "Error: Duplicate key.\
         \n  ,\
         \n6 |   $a: 1,\
         \n  |   == first key\
         \n7 |   $b: 2\
         \n  |   ^^ second key\
         \n  \'\
         \n  input.scss 7:3  root stylesheet",
    );
}
