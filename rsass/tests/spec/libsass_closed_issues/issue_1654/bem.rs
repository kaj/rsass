//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1654/bem.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("bem")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%foo,\
             \n.foo {\
             \n display:block;\n\
             \n  &--up {\
             \n   border: none;\
             \n }\
             \n}\n\
             \n.zoo {\
             \n  @extend %foo;\n\
             \n  &--up {\
             \n    @extend %foo--up;\
             \n  }\
             \n}"),
        ".zoo,\
         \n.foo {\
         \n  display: block;\
         \n}\
         \n.zoo--up,\
         \n.foo--up {\
         \n  border: none;\
         \n}\n"
    );
}
