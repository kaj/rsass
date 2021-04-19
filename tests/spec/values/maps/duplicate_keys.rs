//! Tests auto-converted from "sass-spec/spec/values/maps/duplicate-keys.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "$map: (\
             \n  alpha: 1,\
             \n  beta: 2,\
             \n  gamma: 3,\
             \n  delta: (\
             \n    eta: 5,\
             \n    eta: 6,\
             \n  ),\
             \n);\
             \n"
        )
        .unwrap_err(),
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
