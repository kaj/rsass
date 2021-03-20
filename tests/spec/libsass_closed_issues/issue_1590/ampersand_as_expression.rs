//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1590/ampersand-as-expression.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
