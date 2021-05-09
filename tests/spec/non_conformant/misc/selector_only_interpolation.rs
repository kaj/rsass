//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/selector_only_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("#{\"foo\" + \" bar\"} {a: b}\n"),
        "foo bar {\
         \n  a: b;\
         \n}\n"
    );
}
