//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/media/nesting/merged.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "// Media queries can be nested within one another. The intersection of the two\
            \n// queries will be generated.\
            \n@media screen {\
            \n  a {b: c}\
            \n  @media (color) {x {y: z}}\
            \n\
            \n  // The \"all and\" prefix shouldn\'t change the semantics.\
            \n  @media all and (color) {x {y: z}}\
            \n}\
            \n\
            \n// Features always go to the end of a query, even if they\'re at an outer nesting\
            \n// level.\
            \n@media (color) {\
            \n  a {b: c}\
            \n  @media screen {x {y: z}}\
            \n}\
            \n@media all and (color) {\
            \n  a {b: c}\
            \n  @media screen {x {y: z}}\
            \n}\
            \n\
            \n// Different features can be intersected.\
            \n@media (max-width: 300px) {\
            \n  a {b: c}\
            \n  @media (min-width: 200px) {x {y: z}}\
            \n  @media all and (min-width: 200px) {q {r: s}}\
            \n}\
            \n@media all and (max-width: 300px) {\
            \n  a {b: c}\
            \n  @media (min-width: 200px) {x {y: z}}\
            \n  @media all and (min-width: 200px) {q {r: s}}\
            \n}\
            \n\
            \n// Unlike `not`, the `only` keyword is preserved through intersection.\
            \n@media only screen {\
            \n  a {b: c}\
            \n  @media (color) {x {y: z}}\
            \n  @media all and (color) {q {r: s}}\
            \n}\
            \n\
            \n// The intersection of `not screen` and `print` is just `print`.\
            \n@media not screen {\
            \n  a {b: c}\
            \n  @media print {x {y: z}}\
            \n}\
            \n@media print {\
            \n  a {b: c}\
            \n  @media not screen {x {y: z}}\
            \n}\
            \n\
            \n// The intersection of `not screen` with `not screen and (color)` is the\
            \n// narrower `not screen and (color)`.\
            \n@media not screen {\
            \n  a {b: c}\
            \n  @media not screen and (color) {x {y: z}}\
            \n}\
            \n@media not screen and (color) {\
            \n  a {b: c}\
            \n  @media not screen {x {y: z}}\
            \n}\
            \n\
            \n// The same is true if `print` has additional features.\
            \n@media not screen {\
            \n  a {b: c}\
            \n  @media print and (color) {x {y: z}}\
            \n}\
            \n@media print and (color) {\
            \n  a {b: c}\
            \n  @media not screen {x {y: z}}\
            \n}\
            \n\
            \n// It\'s also true if `screen` has additional features.\
            \n@media not screen and (color) {\
            \n  a {b: c}\
            \n  @media print {x {y: z}}\
            \n}\
            \n@media print {\
            \n  a {b: c}\
            \n  @media not screen and (color) {x {y: z}}\
            \n}\
            \n\
            \n// If a rule has multiple queries and they\'re all mergeable, merge them all as a\
            \n// cross-product.\
            \n@media screen, print {\
            \n  a {b: c}\
            \n  @media (color), (grid) {\
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
        \n@media screen and (color) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media screen and (color) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media screen and (color) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media all and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media screen and (color) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media (max-width: 300px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (max-width: 300px) and (min-width: 200px) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media (max-width: 300px) and (min-width: 200px) {\
        \n  q {\
        \n    r: s;\
        \n  }\
        \n}\
        \n@media all and (max-width: 300px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media (max-width: 300px) and (min-width: 200px) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media all and (max-width: 300px) and (min-width: 200px) {\
        \n  q {\
        \n    r: s;\
        \n  }\
        \n}\
        \n@media only screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media only screen and (color) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media only screen and (color) {\
        \n  q {\
        \n    r: s;\
        \n  }\
        \n}\
        \n@media not screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media print {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media print {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media print {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media not screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media not screen and (color) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media not screen and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media not screen and (color) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media not screen {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media print and (color) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media print and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media print and (color) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media not screen and (color) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media print {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media print {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media print {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n@media screen, print {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n@media screen and (color), screen and (grid), print and (color), print and (grid) {\
        \n  x {\
        \n    y: z;\
        \n  }\
        \n}\
        \n"
    );
}
