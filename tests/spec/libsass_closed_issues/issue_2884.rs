//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2884.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$titles: \"foo\", \"bar\", \"BaZ\";\
            \n\
            \n%border {\
            \n        border: 1px solid;\
            \n}\
            \n\
            \n@mixin border-red {\
            \n        border-color: red;\
            \n}\
            \n\
            \n@mixin border-blue {\
            \n        border-color: blue;\
            \n}\
            \n\
            \n@each $t in $titles {\
            \n        p[title=\"#{$t}\" i] {\
            \n                @extend %border;\
            \n                @include border-red;\
            \n        }\
            \n        p[title=\"#{$t}\"] {\
            \n                @extend %border;\
            \n                @include border-blue;\
            \n        }\
            \n}\
            \n"
        )
        .unwrap(),
        "p[title=BaZ], p[title=BaZ i], p[title=bar], p[title=bar i], p[title=foo], p[title=foo i] {\
        \n  border: 1px solid;\
        \n}\
        \np[title=foo i] {\
        \n  border-color: red;\
        \n}\
        \np[title=foo] {\
        \n  border-color: blue;\
        \n}\
        \np[title=bar i] {\
        \n  border-color: red;\
        \n}\
        \np[title=bar] {\
        \n  border-color: blue;\
        \n}\
        \np[title=BaZ i] {\
        \n  border-color: red;\
        \n}\
        \np[title=BaZ] {\
        \n  border-color: blue;\
        \n}\
        \n"
    );
}
