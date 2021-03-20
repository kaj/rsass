//! Tests auto-converted from "sass-spec/spec/css/media/range/with_expressions.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "// Media query range syntax with SassScript expressions.\
            \n\
            \n// Single comparisons.\
            \n$width: width;\
            \n@media ($width < 600px) {a {dynamic: left}}\
            \n@media (width < 500px + 100px) {a {dynamic: right}}\
            \n@media ($width < 500px + 100px) {a {dynamic: both}}\
            \n@media ($width = 500px + 100px) {a {separator: equals}}\
            \n\
            \n// Ranges.\
            \n@media (50px + 50px < width < 600px) {a {dynamic: left}}\
            \n@media (100px < $width < 600px) {a {dynamic: middle}}\
            \n@media (100px < width < 500px + 100px) {a {dynamic: right}}\
            \n@media (50px + 50px < $width < 500px + 100px) {a {dynamic: all}}\
            \n\
            \n// Comparison operators are allowed as long as they\'re not at the top level.\
            \n@media (width < (1 < 2)) {a {comparison: in-operator}}\
            \n@media (width < if(1 < 2, 500px, 600px)) {a {comparison: in-function}}\
            \n@media (width < [1 < 2]) {a {comparison: in-square-brackets}}\
            \n"
        )
        .unwrap(),
        "@media (width < 600px) {\
        \n  a {\
        \n    dynamic: left;\
        \n  }\
        \n}\
        \n@media (width < 600px) {\
        \n  a {\
        \n    dynamic: right;\
        \n  }\
        \n}\
        \n@media (width < 600px) {\
        \n  a {\
        \n    dynamic: both;\
        \n  }\
        \n}\
        \n@media (width = 600px) {\
        \n  a {\
        \n    separator: equals;\
        \n  }\
        \n}\
        \n@media (100px < width < 600px) {\
        \n  a {\
        \n    dynamic: left;\
        \n  }\
        \n}\
        \n@media (100px < width < 600px) {\
        \n  a {\
        \n    dynamic: middle;\
        \n  }\
        \n}\
        \n@media (100px < width < 600px) {\
        \n  a {\
        \n    dynamic: right;\
        \n  }\
        \n}\
        \n@media (100px < width < 600px) {\
        \n  a {\
        \n    dynamic: all;\
        \n  }\
        \n}\
        \n@media (width < true) {\
        \n  a {\
        \n    comparison: in-operator;\
        \n  }\
        \n}\
        \n@media (width < 500px) {\
        \n  a {\
        \n    comparison: in-function;\
        \n  }\
        \n}\
        \n@media (width < [true]) {\
        \n  a {\
        \n    comparison: in-square-brackets;\
        \n  }\
        \n}\
        \n"
    );
}
