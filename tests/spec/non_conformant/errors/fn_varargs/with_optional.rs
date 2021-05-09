//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-varargs/with-optional.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function test($param:\"default\",$rest...) {}"),
        ""
    );
}
