//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/directives-in-propsets.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$color: red;\
            \n$position: 50%;\
            \n$x: 0;\
            \n\
            \n@mixin foo() {\
            \n  image: url(foo.png);\
            \n}\
            \n\
            \ndiv {\
            \n  background: {\
            \n    something: {\
            \n      color: green;\
            \n    }\
            \n    @if (type-of($color) == \"color\") {\
            \n      color: $color;\
            \n    }\
            \n    @if (type-of($position) == \"number\") {\
            \n      position: $position;\
            \n      @include foo();\
            \n    }\
            \n    groo: foo;\
            \n  }\
            \n  width: $x;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background-something-color: green;\
        \n  background-color: red;\
        \n  background-position: 50%;\
        \n  background-image: url(foo.png);\
        \n  background-groo: foo;\
        \n  width: 0;\
        \n}\
        \n"
    );
}
