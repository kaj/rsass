//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/119_test_basic_prop_name_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {bar#{\"baz\" + \"bang\"}: blip}\n"),
        "foo {\
         \n  barbazbang: blip;\
         \n}\n"
    );
}
