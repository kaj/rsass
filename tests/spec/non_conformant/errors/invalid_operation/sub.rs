//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-operation/sub.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("sub")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "test {\r\
             \n  err: 2px - 2px*2px;\r\
             \n}"
        ),
        "Error: 2px and 4px*px have incompatible units.\
         \n  ,\
         \n2 |   err: 2px - 2px*2px;\
         \n  |        ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet",
    );
}
