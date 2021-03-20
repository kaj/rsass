//! Tests auto-converted from "sass-spec/spec/libsass/keyframes.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  color: #181818;\
            \n}\
            \n\
            \n@-webkit-keyframes uiDelayedFadeIn {\
            \n\t0% { opacity: 0; }\
            \n\t50% { opacity: .5; }\
            \n\t100% { opacity: 1; }\
            \n}\
            \n\
            \n@-webkit-keyframes bounce {\
            \n\tfrom {\
            \n\t\tleft: 0px;\
            \n\t}\
            \n\tto {\
            \n\t\tleft: 200px;\
            \n\t}\
            \n}\
            \n\
            \n$name: bounce;\
            \n\
            \n@-webkit-keyframes #{$name} {\
            \n  blah: blee;\
            \n}\
            \n\
            \n@mixin fudge() {\
            \n  @content;\
            \n}\
            \n\
            \nfoo {\
            \n  @include fudge() {\
            \n    div {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  color: #181818;\
        \n}\
        \n@-webkit-keyframes uiDelayedFadeIn {\
        \n  0% {\
        \n    opacity: 0;\
        \n  }\
        \n  50% {\
        \n    opacity: 0.5;\
        \n  }\
        \n  100% {\
        \n    opacity: 1;\
        \n  }\
        \n}\
        \n@-webkit-keyframes bounce {\
        \n  from {\
        \n    left: 0px;\
        \n  }\
        \n  to {\
        \n    left: 200px;\
        \n  }\
        \n}\
        \n@-webkit-keyframes bounce {\
        \n  blah: blee;\
        \n}\
        \nfoo div {\
        \n  color: red;\
        \n}\
        \n"
    );
}
