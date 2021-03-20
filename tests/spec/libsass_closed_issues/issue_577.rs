//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_577.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function map-each($map) {\
            \n  $values: ();\
            \n\
            \n  @each $key, $value in $map {\
            \n    $values: append($values, $value);\
            \n  }\
            \n\
            \n  @return $values;\
            \n}\
            \n\
            \n$map: (foo: bar);\
            \n\
            \n.test {\
            \n  -map-test: map-each($map);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  -map-test: bar;\
        \n}\
        \n"
    );
}
