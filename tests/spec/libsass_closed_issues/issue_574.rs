//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_574.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$flow: left;\
            \n\
            \n$map: (\
            \n  margin-#{$flow}: 3em,\
            \n  foo: bar,\
            \n);\
            \n\
            \n.test {\
            \n  margin-left: map-get($map, margin-left);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  margin-left: 3em;\
        \n}\
        \n"
    );
}
