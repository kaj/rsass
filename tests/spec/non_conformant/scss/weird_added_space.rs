//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/weird_added_space.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
