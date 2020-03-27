//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1803"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1803/nested.hrx"

// Ignoring "nested", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1803/shallow.hrx"
#[test]
fn shallow() {
    assert_eq!(
        rsass(
            "a {\
            \n  display: block\
            \n\
            \n  b {\
            \n    foo: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  display: block b;\
        \n  display-foo: bar;\
        \n}\
        \n"
    );
}
