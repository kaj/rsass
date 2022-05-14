//! Tests auto-converted from "sass-spec/spec/libsass/selectors/variables/multiple/bare.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("bare")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo a,\
             \n.bar p {\
             \n  $bar: &;\
             \n  content: $bar;\
             \n}"),
        ".foo a,\
         \n.bar p {\
         \n  content: .foo a, .bar p;\
         \n}\n"
    );
}
