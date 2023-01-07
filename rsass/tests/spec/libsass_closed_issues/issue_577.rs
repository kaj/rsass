//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_577.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_577")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@function map-each($map) {\
             \n  $values: ();\n\
             \n  @each $key, $value in $map {\
             \n    $values: append($values, $value);\
             \n  }\n\
             \n  @return $values;\
             \n}\n\
             \n$map: (foo: bar);\n\
             \n.test {\
             \n  -map-test: map-each($map);\
             \n}\n"),
        ".test {\
         \n  -map-test: bar;\
         \n}\n"
    );
}
