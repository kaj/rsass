//! Tests auto-converted from "sass-spec/spec/libsass/selectors/variables/single/interpolated.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  $bar: &;\
             \n  content: #{$bar};\
             \n}"),
        ".foo {\
         \n  content: .foo;\
         \n}\n"
    );
}
