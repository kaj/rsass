//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_659/issue_659.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "// libsass issue 659: never output empty blocks\
            \n// https://github.com/sass/libsass/issues/659\
            \n\
            \n@function null() {\
            \n  @return null;\
            \n}\
            \n\
            \n$foo: null;\
            \n\
            \n.test {\
            \n  out: null();\
            \n  out: $foo;\
            \n}"
        )
        .unwrap(),
        ""
    );
}
