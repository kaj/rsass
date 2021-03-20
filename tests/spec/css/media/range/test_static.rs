//! Tests auto-converted from "sass-spec/spec/css/media/range/static.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "// Plain CSS examples of the media query range syntax.\
            \n\
            \n// Test all the comparison operators.\
            \n@media (height < 600px) {a {b: c}}\
            \n@media (height <= 600px) {a {b: c}}\
            \n@media (height = 600px) {a {b: c}}\
            \n@media (height >= 600px) {a {b: c}}\
            \n@media (height > 600px) {a {b: c}}\
            \n\
            \n// Test all the range operators.\
            \n@media (10px < width < 15px) {a {b: c}}\
            \n@media (10px < width <= 15px) {a {b: c}}\
            \n@media (10px <= width < 15px) {a {b: c}}\
            \n@media (10px <= width <= 15px) {a {b: c}}\
            \n@media (10px > width > 15px) {a {b: c}}\
            \n@media (10px > width >= 15px) {a {b: c}}\
            \n@media (10px >= width > 15px) {a {b: c}}\
            \n@media (10px >= width >= 15px) {a {b: c}}\
            \n\
            \n// Ratio syntax should fall out from Sass\'s existing rules for handling `/`.\
            \n@media (device-aspect-ratio > 3/2) {a {b: c}}\
            \n@media (3/2 < device-aspect-ratio < 16/9) {a {b: c}}\
            \n"
        )
        .unwrap(),
        "@media (height < 600px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (height <= 600px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (height = 600px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (height >= 600px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (height > 600px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px < width < 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px < width <= 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px <= width < 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px <= width <= 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px > width > 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px > width >= 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px >= width > 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (10px >= width >= 15px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (device-aspect-ratio > 3/2) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (3/2 < device-aspect-ratio < 16/9) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n"
    );
}
