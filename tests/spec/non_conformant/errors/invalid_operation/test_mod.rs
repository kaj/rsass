//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-operation/mod.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\r\
             \n  err: 2px % red;\r\
             \n}"
        )
        .unwrap_err(),
        "Error: Undefined operation \"2px % red\".\
         \n  ,\
         \n2 |   err: 2px % red;\
         \n  |        ^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet",
    );
}
