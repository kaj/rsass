//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/121_test_prop_name_only_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("121_test_prop_name_only_interpolation")
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
