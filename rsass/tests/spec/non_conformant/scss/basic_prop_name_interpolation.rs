//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/basic_prop_name_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("basic_prop_name_interpolation")
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
