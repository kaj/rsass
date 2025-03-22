//! Tests auto-converted from "sass-spec/spec/libsass/selector_interpolation_in_string.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector_interpolation_in_string")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo[val=\"bar #{\"foo\" + \" bar\"} baz\"] {a: b}\n"),
        "foo[val=\"bar foo bar baz\"] {\
         \n  a: b;\
         \n}\n"
    );
}
