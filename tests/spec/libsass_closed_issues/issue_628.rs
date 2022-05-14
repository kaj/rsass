//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_628.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_628")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "$map: (\r\
             \n  alpha: 1,\r\
             \n  beta: 2,\r\
             \n  gamma: 3,\r\
             \n  delta: (\r\
             \n    eta: 5,\r\
             \n    eta: 6,\r\
             \n  ),\r\
             \n);"
        ),
        "Error: Duplicate key.\
         \n  ,\
         \n6 |     eta: 5,\
         \n  |     === first key\
         \n7 |     eta: 6,\
         \n  |     ^^^ second key\
         \n  \'\
         \n  input.scss 7:5  root stylesheet",
    );
}
