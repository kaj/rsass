//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/content/recursive.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin span($i) {\
            \n  x: y;\
            \n  @content;\
            \n}\
            \n\
            \n.a {\
            \n  @include span(5) {\
            \n    .inner { @include span(2); }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".a {\
        \n  x: y;\
        \n}\
        \n.a .inner {\
        \n  x: y;\
        \n}\
        \n"
    );
}
