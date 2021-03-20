//! Tests auto-converted from "sass-spec/spec/libsass/mixins-and-media-queries.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@media screen and (orientation:landscape) {\
            \n  span {\
            \n    background: blue;\
            \n  }\
            \n  /* fudge */\
            \n  // @include foo;\
            \n  /* budge */\
            \n  div {\
            \n    color: red;\
            \n  }\
            \n}\
            \n\
            \n@mixin testComments {\
            \n  /* crash */\
            \n  p {\
            \n    width: 100px;\
            \n  }\
            \n}\
            \n\
            \n@media screen and (orientation:landscape) {\
            \n  @include testComments;\
            \n}"
        )
        .unwrap(),
        "@media screen and (orientation: landscape) {\
        \n  span {\
        \n    background: blue;\
        \n  }\
        \n  /* fudge */\
        \n  /* budge */\
        \n  div {\
        \n    color: red;\
        \n  }\
        \n}\
        \n@media screen and (orientation: landscape) {\
        \n  /* crash */\
        \n  p {\
        \n    width: 100px;\
        \n  }\
        \n}\
        \n"
    );
}
