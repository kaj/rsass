//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/media/nesting/retained.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "// There\'s no way to generate the intersection of these queries. We could write\
            \n// `not screen and (color)`, but that actually means \"neither `screen` nor\
            \n// `(color)`\" rather than \"not `screen` but yes `(color)`. However, because they\
            \n// do *have* a meaningful intersection, we output them nested for browsers that\
            \n// support nesting natively.\
            \n//\
            \n// The latest spec allows us to generate `not screen and not (color)` here,\
            \n// which would work, but no browsers support it yet.\
            \n@media not screen {\
            \n  a {b: c}\
            \n  @media (color) {x {y: z}}\
            \n\
            \n  // The \"all and\" prefix shouldn\'t change the semantics.\
            \n  @media all and (color) {q {r: s}}\
            \n}\
            \n@media (color) {\
            \n  a {b: c}\
            \n  @media not screen {x {y: z}}\
            \n}\
            \n@media all and (color) {\
            \n  a {b: c}\
            \n  @media not screen {x {y: z}}\
            \n}\
            \n\
            \n// The unification of these would be `screen and not (color)`, which isn\'t yet\
            \n// supported.\
            \n@media screen {\
            \n  a {b: c}\
            \n  @media not all and (color) {x {y: z}}\
            \n}\
            \n@media not all and (color) {\
            \n  a {b: c}\
            \n  @media screen {x {y: z}}\
            \n}\
            \n\
            \n// `not screen and (color)` means `not (screen and (color))`, so it could still\
            \n// overlap with just `screen` in the case of a screen media without color.\
            \n@media not screen and (color) {\
            \n  a {b: c}\
            \n  @media screen {x {y: z}}\
            \n}\
            \n@media screen {\
            \n  a {b: c}\
            \n  @media not screen and (color) {x {y: z}}\
            \n}\
            \n\
            \n// `not screen and (color)` and `screen and (grid)` are both true for screen\
            \n// user agents with a grid output device and no color support.\
            \n@media not screen and (color) {\
            \n  a {b: c}\
            \n  @media screen and (grid) {x {y: z}}\
            \n}\
            \n@media screen and (grid) {\
            \n  a {b: c}\
            \n  @media not screen and (color) {x {y: z}}\
            \n}\
            \n\
            \n// `not screen` and `not print` allows any media type other than those.\
            \n@media not screen {\
            \n  a {b: c}\
            \n  @media not print {x {y: z}}\
            \n}\
            \n\
            \n// `not screen and (color)` and `not screen and (grid)` allows screen media, but\
            \n// only if it has *neither* color nor grid support.\
            \n@media not screen and (color) {\
            \n  a {b: c}\
            \n  @media not screen and (grid) {x {y: z}}\
            \n}\
            \n\
            \n// If a rule has multiple queries and any of them can\'t be merged, none of them\
            \n// should be. This avoids duplicating the output and ensures that all code is\
            \n// evaluated in a unique media query context in case we ever provide access to\
            \n// that.\
            \n@media screen, not screen {\
            \n  a {b: c}\
            \n  @media (color) {x {y: z}}\
            \n}\
            \n"
        )
        .unwrap(),
        "@media not screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @media (color) {\
        \n    x {\
        \n      y: z;\
        \n    }\
        \n  }\
        \n  @media all and (color) {\
        \n    q {\
        \n      r: s;\
        \n    }\
        \n  }\
        \n}\
        \n@media (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @media not screen {\
        \n    x {\
        \n      y: z;\
        \n    }\
        \n  }\
        \n}\
        \n@media all and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @media not screen {\
        \n    x {\
        \n      y: z;\
        \n    }\
        \n  }\
        \n}\
        \n@media screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @media not all and (color) {\
        \n    x {\
        \n      y: z;\
        \n    }\
        \n  }\
        \n}\
        \n@media not all and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @media screen {\
        \n    x {\
        \n      y: z;\
        \n    }\
        \n  }\
        \n}\
        \n@media not screen and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @media screen {\
        \n    x {\
        \n      y: z;\
        \n    }\
        \n  }\
        \n}\
        \n@media screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @media not screen and (color) {\
        \n    x {\
        \n      y: z;\
        \n    }\
        \n  }\
        \n}\
        \n@media not screen and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @media screen and (grid) {\
        \n    x {\
        \n      y: z;\
        \n    }\
        \n  }\
        \n}\
        \n@media screen and (grid) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @media not screen and (color) {\
        \n    x {\
        \n      y: z;\
        \n    }\
        \n  }\
        \n}\
        \n@media not screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @media not print {\
        \n    x {\
        \n      y: z;\
        \n    }\
        \n  }\
        \n}\
        \n@media not screen and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @media not screen and (grid) {\
        \n    x {\
        \n      y: z;\
        \n    }\
        \n  }\
        \n}\
        \n@media screen, not screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @media (color) {\
        \n    x {\
        \n      y: z;\
        \n    }\
        \n  }\
        \n}\
        \n"
    );
}
