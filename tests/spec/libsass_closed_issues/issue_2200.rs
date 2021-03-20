//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2200.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".media-object-section:last-child:not(:nth-child(2)) {\
            \n  color: blue;\
            \n}\
            \n\
            \n%orbit-control {\
            \n  color: red;\
            \n}\
            \n\
            \n.orbit-previous {\
            \n  @extend %orbit-control;\
            \n}\
            \n"
        )
        .unwrap(),
        ".media-object-section:last-child:not(:nth-child(2)) {\
        \n  color: blue;\
        \n}\
        \n.orbit-previous {\
        \n  color: red;\
        \n}\
        \n"
    );
}
