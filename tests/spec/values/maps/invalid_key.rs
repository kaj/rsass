//! Tests auto-converted from "sass-spec/spec/values/maps/invalid-key.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "$id: inspect((a,b:c)...)\
             \n"
        )
        .unwrap_err(),
        "Error: expected \")\".\
         \n  ,\
         \n1 | $id: inspect((a,b:c)...)\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet\
         \n",
    );
}
