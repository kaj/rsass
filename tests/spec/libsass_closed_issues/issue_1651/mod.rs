//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1651"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1651/with.hrx"

// Ignoring "with", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1651/without.hrx"
#[test]
#[ignore] // unexepected error
fn without() {
    assert_eq!(
        rsass(
            ".a {\
            \n  display: block;\
            \n}\
            \n\
            \n.b {\
            \n  @at-root (without: media) {\
            \n    @extend .a;\
            \n  }\
            \n} \
            \n"
        )
        .unwrap(),
        ".a, .b {\
        \n  display: block;\
        \n}\
        \n"
    );
}
