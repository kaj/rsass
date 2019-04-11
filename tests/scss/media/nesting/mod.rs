//! Tests auto-converted from "sass-spec/spec/scss/media/nesting"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/scss/media/nesting/merged.hrx"
#[test]
#[ignore] // failing
fn merged() {
    assert_eq!(
        rsass(
            "// Media queries can be nested within one another. The intersection of the two\n// queries will be generated.\n@media screen {\n  a {b: c}\n  @media (color) {x {y: z}}\n\n  // The \"all and\" prefix shouldn\'t change the semantics.\n  @media all and (color) {x {y: z}}\n}\n\n// Features always go to the end of a query, even if they\'re at an outer nesting\n// level.\n@media (color) {\n  a {b: c}\n  @media screen {x {y: z}}\n}\n@media all and (color) {\n  a {b: c}\n  @media screen {x {y: z}}\n}\n\n// Different features can be intersected.\n@media (max-width: 300px) {\n  a {b: c}\n  @media (min-width: 200px) {x {y: z}}\n  @media all and (min-width: 200px) {q {r: s}}\n}\n@media all and (max-width: 300px) {\n  a {b: c}\n  @media (min-width: 200px) {x {y: z}}\n  @media all and (min-width: 200px) {q {r: s}}\n}\n\n// Unlike `not`, the `only` keyword is preserved through intersection.\n@media only screen {\n  a {b: c}\n  @media (color) {x {y: z}}\n  @media all and (color) {q {r: s}}\n}\n\n// The intersection of `not screen` and `print` is just `print`.\n@media not screen {\n  a {b: c}\n  @media print {x {y: z}}\n}\n@media print {\n  a {b: c}\n  @media not screen {x {y: z}}\n}\n\n// The intersection of `not screen` with `not screen and (color)` is the\n// narrower `not screen and (color)`.\n@media not screen {\n  a {b: c}\n  @media not screen and (color) {x {y: z}}\n}\n@media not screen and (color) {\n  a {b: c}\n  @media not screen {x {y: z}}\n}\n\n// The same is true if `print` has additional features.\n@media not screen {\n  a {b: c}\n  @media print and (color) {x {y: z}}\n}\n@media print and (color) {\n  a {b: c}\n  @media not screen {x {y: z}}\n}\n\n// It\'s also true if `screen` has additional features.\n@media not screen and (color) {\n  a {b: c}\n  @media print {x {y: z}}\n}\n@media print {\n  a {b: c}\n  @media not screen and (color) {x {y: z}}\n}\n\n// If a rule has multiple queries and they\'re all mergeable, merge them all as a\n// cross-product.\n@media screen, print {\n  a {b: c}\n  @media (color), (grid) {\n    x {y: z};\n  }\n}\n"
        )
        .unwrap(),
        "@media screen {\n  a {\n    b: c;\n  }\n}\n@media screen and (color) {\n  x {\n    y: z;\n  }\n}\n@media screen and (color) {\n  x {\n    y: z;\n  }\n}\n@media (color) {\n  a {\n    b: c;\n  }\n}\n@media screen and (color) {\n  x {\n    y: z;\n  }\n}\n@media all and (color) {\n  a {\n    b: c;\n  }\n}\n@media screen and (color) {\n  x {\n    y: z;\n  }\n}\n@media (max-width: 300px) {\n  a {\n    b: c;\n  }\n}\n@media (max-width: 300px) and (min-width: 200px) {\n  x {\n    y: z;\n  }\n}\n@media (max-width: 300px) and (min-width: 200px) {\n  q {\n    r: s;\n  }\n}\n@media all and (max-width: 300px) {\n  a {\n    b: c;\n  }\n}\n@media (max-width: 300px) and (min-width: 200px) {\n  x {\n    y: z;\n  }\n}\n@media all and (max-width: 300px) and (min-width: 200px) {\n  q {\n    r: s;\n  }\n}\n@media only screen {\n  a {\n    b: c;\n  }\n}\n@media only screen and (color) {\n  x {\n    y: z;\n  }\n}\n@media only screen and (color) {\n  q {\n    r: s;\n  }\n}\n@media not screen {\n  a {\n    b: c;\n  }\n}\n@media print {\n  x {\n    y: z;\n  }\n}\n@media print {\n  a {\n    b: c;\n  }\n}\n@media print {\n  x {\n    y: z;\n  }\n}\n@media not screen {\n  a {\n    b: c;\n  }\n}\n@media not screen and (color) {\n  x {\n    y: z;\n  }\n}\n@media not screen and (color) {\n  a {\n    b: c;\n  }\n}\n@media not screen and (color) {\n  x {\n    y: z;\n  }\n}\n@media not screen {\n  a {\n    b: c;\n  }\n}\n@media print and (color) {\n  x {\n    y: z;\n  }\n}\n@media print and (color) {\n  a {\n    b: c;\n  }\n}\n@media print and (color) {\n  x {\n    y: z;\n  }\n}\n@media not screen and (color) {\n  a {\n    b: c;\n  }\n}\n@media print {\n  x {\n    y: z;\n  }\n}\n@media print {\n  a {\n    b: c;\n  }\n}\n@media print {\n  x {\n    y: z;\n  }\n}\n@media screen, print {\n  a {\n    b: c;\n  }\n}\n@media screen and (color), screen and (grid), print and (color), print and (grid) {\n  x {\n    y: z;\n  }\n}\n"
    );
}

// From "sass-spec/spec/scss/media/nesting/merged_and_retained.hrx"
#[test]
#[ignore] // failing
fn merged_and_retained() {
    assert_eq!(
        rsass(
            "@media (retained: before) {\n  a {b: c}\n  @media (a: b) {\n    x {y: z}\n  }\n}\n\n// Regression test for sass/dart-sass#453\n@media (retained: after) {\n  @media (a: b) {\n    x {y: z}\n  }\n  a {b: c}\n}\n"
        )
        .unwrap(),
        "@media (retained: before) {\n  a {\n    b: c;\n  }\n}\n@media (retained: before) and (a: b) {\n  x {\n    y: z;\n  }\n}\n@media (retained: after) and (a: b) {\n  x {\n    y: z;\n  }\n}\n@media (retained: after) {\n  a {\n    b: c;\n  }\n}\n"
    );
}

// From "sass-spec/spec/scss/media/nesting/removed.hrx"
#[test]
#[ignore] // failing
fn removed() {
    assert_eq!(
        rsass(
            "// The intersection of two different media types is empty, so they\'re eliminated.\n@media screen {\n  a {b: c}\n  @media print {x {y: z}}\n}\n\n// The intersection of `not screen` and `screen` is empty.\n@media not screen {\n  a {b: c}\n  @media screen {x {y: z}}\n}\n@media screen {\n  a {b: c}\n  @media not screen {x {y: z}}\n}\n\n// That\'s true even if `screen` has features.\n@media screen and (color) {\n  a {b: c}\n  @media not screen {x {y: z}}\n}\n@media not screen {\n  a {b: c}\n  @media screen and (color) {x {y: z}}\n}\n\n// In fact, the intersection of `not X` and `X` is empty for all `X`.\n@media not screen and (color) {\n  a {b: c}\n  @media screen and (color) {x {y: z}}\n}\n@media screen and (color) {\n  a {b: c}\n  @media not screen and (color) {x {y: z}}\n}\n\n// This intersection is empty even though the queries aren\'t identical, because\n// `not screen` matches a superset of the contexts `screen and (color)` matches.\n@media screen and (color) {\n  a {b: c}\n  @media not screen {x {y: z}}\n}\n@media not screen {\n  a {b: c}\n  @media screen and (color) {x {y: z}}\n}\n\n// If a rule has multiple queries and some have empty intersections, remove them\n// and merge the rest.\n@media screen, print {\n  a {b: c}\n  @media speech, (grid) {\n    x {y: z};\n  }\n}\n"
        )
        .unwrap(),
        "@media screen {\n  a {\n    b: c;\n  }\n}\n@media not screen {\n  a {\n    b: c;\n  }\n}\n@media screen {\n  a {\n    b: c;\n  }\n}\n@media screen and (color) {\n  a {\n    b: c;\n  }\n}\n@media not screen {\n  a {\n    b: c;\n  }\n}\n@media not screen and (color) {\n  a {\n    b: c;\n  }\n}\n@media screen and (color) {\n  a {\n    b: c;\n  }\n}\n@media screen and (color) {\n  a {\n    b: c;\n  }\n}\n@media not screen {\n  a {\n    b: c;\n  }\n}\n@media screen, print {\n  a {\n    b: c;\n  }\n}\n@media screen and (grid), print and (grid) {\n  x {\n    y: z;\n  }\n}\n"
    );
}

// From "sass-spec/spec/scss/media/nesting/retained.hrx"
#[test]
#[ignore] // failing
fn retained() {
    assert_eq!(
        rsass(
            "// There\'s no way to generate the intersection of these queries. We could write\n// `not screen and (color)`, but that actually means \"neither `screen` nor\n// `(color)`\" rather than \"not `screen` but yes `(color)`. However, because they\n// do *have* a meaningful intersection, we output them nested for browsers that\n// support nesting natively.\n//\n// The latest spec allows us to generate `not screen and not (color)` here,\n// which would work, but no browsers support it yet.\n@media not screen {\n  a {b: c}\n  @media (color) {x {y: z}}\n\n  // The \"all and\" prefix shouldn\'t change the semantics.\n  @media all and (color) {q {r: s}}\n}\n@media (color) {\n  a {b: c}\n  @media not screen {x {y: z}}\n}\n@media all and (color) {\n  a {b: c}\n  @media not screen {x {y: z}}\n}\n\n// The unification of these would be `screen and not (color)`, which isn\'t yet\n// supported.\n@media screen {\n  a {b: c}\n  @media not all and (color) {x {y: z}}\n}\n@media not all and (color) {\n  a {b: c}\n  @media screen {x {y: z}}\n}\n\n// `not screen and (color)` means `not (screen and (color))`, so it could still\n// overlap with just `screen` in the case of a screen media without color.\n@media not screen and (color) {\n  a {b: c}\n  @media screen {x {y: z}}\n}\n@media screen {\n  a {b: c}\n  @media not screen and (color) {x {y: z}}\n}\n\n// `not screen and (color)` and `screen and (grid)` are both true for screen\n// user agents with a grid output device and no color support.\n@media not screen and (color) {\n  a {b: c}\n  @media screen and (grid) {x {y: z}}\n}\n@media screen and (grid) {\n  a {b: c}\n  @media not screen and (color) {x {y: z}}\n}\n\n// `not screen` and `not print` allows any media type other than those.\n@media not screen {\n  a {b: c}\n  @media not print {x {y: z}}\n}\n\n// `not screen and (color)` and `not screen and (grid)` allows screen media, but\n// only if it has *neither* color nor grid support.\n@media not screen and (color) {\n  a {b: c}\n  @media not screen and (grid) {x {y: z}}\n}\n\n// If a rule has multiple queries and any of them can\'t be merged, none of them\n// should be. This avoids duplicating the output and ensures that all code is\n// evaluated in a unique media query context in case we ever provide access to\n// that.\n@media screen, not screen {\n  a {b: c}\n  @media (color) {x {y: z}}\n}\n"
        )
        .unwrap(),
        "@media not screen {\n  a {\n    b: c;\n  }\n  @media (color) {\n    x {\n      y: z;\n    }\n  }\n  @media all and (color) {\n    q {\n      r: s;\n    }\n  }\n}\n@media (color) {\n  a {\n    b: c;\n  }\n  @media not screen {\n    x {\n      y: z;\n    }\n  }\n}\n@media all and (color) {\n  a {\n    b: c;\n  }\n  @media not screen {\n    x {\n      y: z;\n    }\n  }\n}\n@media screen {\n  a {\n    b: c;\n  }\n  @media not all and (color) {\n    x {\n      y: z;\n    }\n  }\n}\n@media not all and (color) {\n  a {\n    b: c;\n  }\n  @media screen {\n    x {\n      y: z;\n    }\n  }\n}\n@media not screen and (color) {\n  a {\n    b: c;\n  }\n  @media screen {\n    x {\n      y: z;\n    }\n  }\n}\n@media screen {\n  a {\n    b: c;\n  }\n  @media not screen and (color) {\n    x {\n      y: z;\n    }\n  }\n}\n@media not screen and (color) {\n  a {\n    b: c;\n  }\n  @media screen and (grid) {\n    x {\n      y: z;\n    }\n  }\n}\n@media screen and (grid) {\n  a {\n    b: c;\n  }\n  @media not screen and (color) {\n    x {\n      y: z;\n    }\n  }\n}\n@media not screen {\n  a {\n    b: c;\n  }\n  @media not print {\n    x {\n      y: z;\n    }\n  }\n}\n@media not screen and (color) {\n  a {\n    b: c;\n  }\n  @media not screen and (grid) {\n    x {\n      y: z;\n    }\n  }\n}\n@media screen, not screen {\n  a {\n    b: c;\n  }\n  @media (color) {\n    x {\n      y: z;\n    }\n  }\n}\n"
    );
}
