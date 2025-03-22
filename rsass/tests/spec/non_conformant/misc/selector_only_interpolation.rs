//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/selector_only_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector_only_interpolation")
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
