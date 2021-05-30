//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/207_test_optional_extend_succeeds_when_extendee_doesnt_exist.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(runner().ok(".foo {@extend .bar !optional}\n"), "");
}
