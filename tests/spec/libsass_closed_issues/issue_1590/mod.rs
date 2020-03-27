//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1590"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1590/ampersand-against-literal.hrx"
#[test]
fn ampersand_against_literal() {
    assert_eq!(
        rsass(
            "@mixin where($sel: null) {\
            \n    @if ( & == $sel ) {\
            \n        h1 { color: white; }\
            \n    } @else {\
            \n        h1 { color: blue; }\
            \n    }\
            \n}\
            \n.hive { @include where(); } \
            \n.bee { @include where(\".bee\"); } \
            \n.amp { @include where(&); } \
            \n.quotedamp { @include where(\"&\"); } \
            \n"
        )
        .unwrap(),
        ".hive h1 {\
        \n  color: blue;\
        \n}\
        \n.bee h1 {\
        \n  color: blue;\
        \n}\
        \n.amp h1 {\
        \n  color: white;\
        \n}\
        \n.quotedamp h1 {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1590/ampersand-as-expression.hrx"
#[test]
fn ampersand_as_expression() {
    assert_eq!(
        rsass(
            "@mixin where($sel: null) {\
            \n    @if ( & == $sel ) {\
            \n        h1 { color: white; }\
            \n    } @else {\
            \n        h1 { color: blue; }\
            \n    }\
            \n}\
            \n.hive { @include where(); } \
            \n.bee { @include where(&); } \
            \n.queen { @include where(\"&\"); } \
            \n"
        )
        .unwrap(),
        ".hive h1 {\
        \n  color: blue;\
        \n}\
        \n.bee h1 {\
        \n  color: white;\
        \n}\
        \n.queen h1 {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1590/quoted-ampersand-does-not-select.hrx"
#[test]
fn quoted_ampersand_does_not_select() {
    assert_eq!(
        rsass(
            "@mixin where($sel: null) {\
            \n    @if ( \"&\" == $sel ) {\
            \n        h1 { color: white; }\
            \n    } @else {\
            \n        h1 { color: blue; }\
            \n    }\
            \n}\
            \n.hive { @include where(); } \
            \n.bee { @include where(&); } \
            \n.amp { @include where(\".amp\"); } \
            \n.queen { @include where(\"&\"); } \
            \n"
        )
        .unwrap(),
        ".hive h1 {\
        \n  color: blue;\
        \n}\
        \n.bee h1 {\
        \n  color: blue;\
        \n}\
        \n.amp h1 {\
        \n  color: blue;\
        \n}\
        \n.queen h1 {\
        \n  color: white;\
        \n}\
        \n"
    );
}
