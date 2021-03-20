//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_736.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "// libsass issue 736:  @return does not cause function exit\
            \n// https://github.com/sass/libsass/issues/736\
            \n\
            \n@function contains-true($list) {\
            \n  @each $bool in $list {\
            \n    @if $bool {\
            \n      @return \"found true\";\
            \n    }\
            \n  }\
            \n  @return \"nothing found\";\
            \n}\
            \n\
            \n.test {\
            \n  out: contains-true(true false false);\
            \n  out: contains-true(false true false);\
            \n  out: contains-true(false false true);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  out: \"found true\";\
        \n  out: \"found true\";\
        \n  out: \"found true\";\
        \n}\
        \n"
    );
}
