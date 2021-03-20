//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/207_test_optional_extend_succeeds_when_extendee_doesnt_exist.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {@extend .bar !optional}\
            \n"
        )
        .unwrap(),
        ""
    );
}
