//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/prop_name_interpolation_after_hyphen.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("prop_name_interpolation_after_hyphen")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a { -#{\"foo\"}-bar: b; }\n"),
        "a {\
         \n  -foo-bar: b;\
         \n}\n"
    );
}
