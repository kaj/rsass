//! Tests auto-converted from "sass-spec/spec/libsass/placeholder-nested.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("placeholder-nested")
}

#[test]
#[ignore] // unexepected error
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
