//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/mixin_content.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$color: blue;\
            \n@mixin context($class, $color: red) {\
            \n  .#{$class} {\
            \n    background-color: $color;\
            \n    @content;\
            \n    border-color: $color;\
            \n  }\
            \n}\
            \n@include context(parent) {\
            \n  @include context(child, $color: yellow) {\
            \n    color: $color;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent {\
        \n  background-color: red;\
        \n  border-color: red;\
        \n}\
        \n.parent .child {\
        \n  background-color: yellow;\
        \n  color: blue;\
        \n  border-color: yellow;\
        \n}\
        \n"
    );
}
