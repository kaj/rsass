//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1757"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_1757/each.hrx"
#[test]
fn each() {
    assert_eq!(
        rsass(
            ".test .nest {\
            \n  length: length(&);\
            \n  @each $list in & {\
            \n    list: $list;\
            \n    length: length($list);\
            \n  }\
            \n}\
            \n\
            \n.test, .other {\
            \n  length: length(&);\
            \n  @each $list in & {\
            \n    list: $list;\
            \n    length: length($list);\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".test .nest {\
        \n  length: 1;\
        \n  list: .test .nest;\
        \n  length: 2;\
        \n}\
        \n.test, .other {\
        \n  length: 2;\
        \n  list: .test;\
        \n  length: 1;\
        \n  list: .other;\
        \n  length: 1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1757/for.hrx"
#[test]
fn test_for() {
    assert_eq!(
        rsass(
            ".test .nest {\
            \n  length: length(&);\
            \n  @for $i from 1 through length(&) {\
            \n    $list: nth(&, $i);\
            \n    list: $list;\
            \n    length: length($list);\
            \n  }\
            \n}\
            \n\
            \n.test, .other {\
            \n  length: length(&);\
            \n  @for $i from 1 through length(&) {\
            \n    $list: nth(&, $i);\
            \n    list: $list;\
            \n    length: length($list);\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".test .nest {\
        \n  length: 1;\
        \n  list: .test .nest;\
        \n  length: 2;\
        \n}\
        \n.test, .other {\
        \n  length: 2;\
        \n  list: .test;\
        \n  length: 1;\
        \n  list: .other;\
        \n  length: 1;\
        \n}\
        \n"
    );
}
