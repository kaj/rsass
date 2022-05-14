//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/css_cdo_and_cdc_ignored_at_toplevel.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_cdo_and_cdc_ignored_at_toplevel")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  bar: baz; }\n\
             \nbar {\
             \n  bar: baz; }\n\
             \nbaz {\
             \n  bar: baz; }\n"),
        "foo {\
         \n  bar: baz;\
         \n}\
         \nbar {\
         \n  bar: baz;\
         \n}\
         \nbaz {\
         \n  bar: baz;\
         \n}\n"
    );
}
