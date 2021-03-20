//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-varargs/with-optional.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass("@function test($param:\"default\",$rest...) {}")
            .unwrap(),
        ""
    );
}
