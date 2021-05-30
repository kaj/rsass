//! Tests auto-converted from "sass-spec/spec/libsass/placeholder-nested.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%x {\
             \n  width: 100px;\n\
             \n  %y {\
             \n    height: 100px;\
             \n  }\
             \n}\n\
             \n.foo {\
             \n  @extend %x;\n\
             \n  .bar { @extend %y }\
             \n}\n"),
        ".foo {\
         \n  width: 100px;\
         \n}\
         \n.foo .bar {\
         \n  height: 100px;\
         \n}\n"
    );
}
