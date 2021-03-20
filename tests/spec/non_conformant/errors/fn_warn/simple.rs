//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-warn/simple.hrx"

#[test]
fn test() {
    assert_eq!(crate::rsass("@warn \"warn\";").unwrap(), "");
}
