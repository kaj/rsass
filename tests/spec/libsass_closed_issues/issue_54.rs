//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_54.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin opacity($percent) {\r\
            \n  foo { test: opacity($percent); }\r\
            \n}\r\
            \n\r\
            \n@-webkit-keyframes uiDelayedFadeIn {\r\
            \n  0% { @include opacity(0.01); }\r\
            \n  50% { @include opacity(0.01); }\r\
            \n  100% { @include opacity(1); }\r\
            \n}\r\
            \n\r\
            \n@-webkit-keyframes bounce {\r\
            \n  from {\r\
            \n    left: 0px;\r\
            \n  }\r\
            \n  to {\r\
            \n    left: 200px;\r\
            \n  }\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "@-webkit-keyframes uiDelayedFadeIn {\
        \n  0% {\
        \n    foo {\
        \n      test: opacity(0.01);\
        \n    }\
        \n  }\
        \n  50% {\
        \n    foo {\
        \n      test: opacity(0.01);\
        \n    }\
        \n  }\
        \n  100% {\
        \n    foo {\
        \n      test: opacity(1);\
        \n    }\
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
        \n"
    );
}
