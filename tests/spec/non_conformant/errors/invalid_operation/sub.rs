//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-operation/sub.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\r\
             \n  err: 2px - 2px*2px;\r\
             \n}"
        )
        .unwrap_err(),
        "Error: 2px and 4px*px have incompatible units.\
         \n  ,\
         \n2 |   err: 2px - 2px*2px;\r\
         \n  |        ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:8  root stylesheet\
         \n",
    );
}
