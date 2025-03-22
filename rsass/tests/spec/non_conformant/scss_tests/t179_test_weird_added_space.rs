//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/179_test_weird_added_space.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("179_test_weird_added_space")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$value : bip;\n\
             \nfoo {\
             \n  bar: -moz-#{$value};\
             \n}\n"),
        "foo {\
         \n  bar: -moz-bip;\
         \n}\n"
    );
}
