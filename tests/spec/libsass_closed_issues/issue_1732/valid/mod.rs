//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/valid"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1732/valid/directive.hrx"
#[test]
fn directive() {
    assert_eq!(
        rsass(
            "@media all {\
            \n  .foo {\
            \n\tcolor: red;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "@media all {\
        \n  .foo {\
        \n    color: red;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1732/valid/keyframe.hrx"
#[test]
fn keyframe() {
    assert_eq!(
        rsass(
            "@keyframes baz {\
            \n  0% { top: 0; bottom: 100; }\
            \n  100% { top: 100; bottom: 0; }\
            \n}"
        )
        .unwrap(),
        "@keyframes baz {\
        \n  0% {\
        \n    top: 0;\
        \n    bottom: 100;\
        \n  }\
        \n  100% {\
        \n    top: 100;\
        \n    bottom: 0;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1732/valid/mixin-call.hrx"
#[test]
fn mixin_call() {
    assert_eq!(
        rsass(
            "@mixin bar() {\
            \n  @content;\
            \n}\
            \n\
            \nfoo {\
            \n  @include bar {\
            \n    color: blue;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1732/valid/mixin-def.hrx"
#[test]
fn mixin_def() {
    assert_eq!(
        rsass(
            "@mixin a {\
            \n  b: c;\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1732/valid/propset.hrx"
#[test]
fn propset() {
    assert_eq!(
        rsass(
            "foo { \
            \n  border: {\
            \n    width: 1px;\
            \n    color: green;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  border-width: 1px;\
        \n  border-color: green;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1732/valid/ruleset.hrx"
#[test]
fn ruleset() {
    assert_eq!(
        rsass(
            "foo { \
            \n  color: green;\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  color: green;\
        \n}\
        \n"
    );
}
