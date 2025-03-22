//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/120_test_basic_prop_name_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("120_test_basic_prop_name_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {bar#{1 + 2}: blip}\n"),
        "foo {\
         \n  bar3: blip;\
         \n}\n"
    );
}
