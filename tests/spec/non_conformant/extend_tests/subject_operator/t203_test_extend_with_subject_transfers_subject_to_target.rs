//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/subject-operator/203_test_extend_with_subject_transfers_subject_to_target.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "a.foo .bar {a: b}\
             \n.bip .bap! {@extend .foo}\
             \n"
        )
        .unwrap_err(),
        "Error: expected \"{\".\
         \n  ,\
         \n2 | .bip .bap! {@extend .foo}\
         \n  |          ^\
         \n  \'\
         \n  input.scss 2:10  root stylesheet\
         \n",
    );
}
