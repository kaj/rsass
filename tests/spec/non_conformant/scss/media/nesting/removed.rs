//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/media/nesting/removed.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "// The intersection of two different media types is empty, so they\'re eliminated.\
            \n@media screen {\
            \n  a {b: c}\
            \n  @media print {x {y: z}}\
            \n}\
            \n\
            \n// The intersection of `not screen` and `screen` is empty.\
            \n@media not screen {\
            \n  a {b: c}\
            \n  @media screen {x {y: z}}\
            \n}\
            \n@media screen {\
            \n  a {b: c}\
            \n  @media not screen {x {y: z}}\
            \n}\
            \n\
            \n// That\'s true even if `screen` has features.\
            \n@media screen and (color) {\
            \n  a {b: c}\
            \n  @media not screen {x {y: z}}\
            \n}\
            \n@media not screen {\
            \n  a {b: c}\
            \n  @media screen and (color) {x {y: z}}\
            \n}\
            \n\
            \n// In fact, the intersection of `not X` and `X` is empty for all `X`.\
            \n@media not screen and (color) {\
            \n  a {b: c}\
            \n  @media screen and (color) {x {y: z}}\
            \n}\
            \n@media screen and (color) {\
            \n  a {b: c}\
            \n  @media not screen and (color) {x {y: z}}\
            \n}\
            \n\
            \n// This intersection is empty even though the queries aren\'t identical, because\
            \n// `not screen` matches a superset of the contexts `screen and (color)` matches.\
            \n@media screen and (color) {\
            \n  a {b: c}\
            \n  @media not screen {x {y: z}}\
            \n}\
            \n@media not screen {\
            \n  a {b: c}\
            \n  @media screen and (color) {x {y: z}}\
            \n}\
            \n\
            \n// If a rule has multiple queries and some have empty intersections, remove them\
            \n// and merge the rest.\
            \n@media screen, print {\
            \n  a {b: c}\
            \n  @media speech, (grid) {\
            \n    x {y: z};\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media not screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media screen and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media not screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media not screen and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media screen and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media screen and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media not screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media screen, print {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media screen and (grid), print and (grid) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n"
    );
}
