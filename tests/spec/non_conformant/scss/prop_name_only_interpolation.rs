//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/prop_name_only_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {#{\"baz\" + \"bang\"}: blip}\n"),
        "foo {\
         \n  bazbang: blip;\
         \n}\n"
    );
}
