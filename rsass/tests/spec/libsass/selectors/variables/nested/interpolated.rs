//! Tests auto-converted from "sass-spec/spec/libsass/selectors/variables/nested/interpolated.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("interpolated")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo a,\
             \n.bar p {\n\
             \n  .baz {\
             \n    $bar: &;\
             \n    content: #{$bar};\
             \n  }\n\
             \n}"),
        ".foo a .baz,\
         \n.bar p .baz {\
         \n  content: .foo a .baz, .bar p .baz;\
         \n}\n"
    );
}
