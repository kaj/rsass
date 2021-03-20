//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_703.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".test-1 {\
            \n  @for $i from 1 through 3 {\
            \n    content: $i;\
            \n  }\
            \n}\
            \n\
            \n.test-2 {\
            \n  @for $i from 3 through 1 {\
            \n    content: $i;\
            \n  }\
            \n}\
            \n\
            \n.test-3 {\
            \n  @for $i from 1 to 3 {\
            \n    content: $i;\
            \n  }\
            \n}\
            \n\
            \n.test-4 {\
            \n  @for $i from 3 to 1 {\
            \n    content: $i;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".test-1 {\
        \n  content: 1;\
        \n  content: 2;\
        \n  content: 3;\
        \n}\
        \n.test-2 {\
        \n  content: 3;\
        \n  content: 2;\
        \n  content: 1;\
        \n}\
        \n.test-3 {\
        \n  content: 1;\
        \n  content: 2;\
        \n}\
        \n.test-4 {\
        \n  content: 3;\
        \n  content: 2;\
        \n}\
        \n"
    );
}
