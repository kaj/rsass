//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/valid/keyframe.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
