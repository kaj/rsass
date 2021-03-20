//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/31_if_in_mixin.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: true;\
            \n\
            \n@mixin foobar() {\
            \n  @if $x {\
            \n    $x: false !global;\
            \n    content: foo;\
            \n  }\
            \n  @else {\
            \n    $x: true !global;\
            \n    content: bar;\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  @include foobar();\
            \n  @include foobar();\
            \n  @include foobar();\
            \n  $x: true !global;\
            \n  @include foobar();\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  content: foo;\
        \n  content: bar;\
        \n  content: foo;\
        \n  content: foo;\
        \n}\
        \n"
    );
}
