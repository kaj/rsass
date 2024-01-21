//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/named.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("named")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "a {b: selector-extend($selector: \"c.d\", $extendee: \"c\", $extender: \"e\")}\n"
        ),
        "a {\
         \n  b: c.d, e.d;\
         \n}\n"
    );
}
