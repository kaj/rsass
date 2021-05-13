//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2200.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".media-object-section:last-child:not(:nth-child(2)) {\
             \n  color: blue;\
             \n}\n\
             \n%orbit-control {\
             \n  color: red;\
             \n}\n\
             \n.orbit-previous {\
             \n  @extend %orbit-control;\
             \n}\n"),
        ".media-object-section:last-child:not(:nth-child(2)) {\
         \n  color: blue;\
         \n}\
         \n.orbit-previous {\
         \n  color: red;\
         \n}\n"
    );
}
