//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1590/ampersand-against-literal.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin where($sel: null) {\
             \n    @if ( & == $sel ) {\
             \n        h1 { color: white; }\
             \n    } @else {\
             \n        h1 { color: blue; }\
             \n    }\
             \n}\
             \n.hive { @include where(); } \
             \n.bee { @include where(\".bee\"); } \
             \n.amp { @include where(&); } \
             \n.quotedamp { @include where(\"&\"); } \n"),
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
         \n}\n"
    );
}
