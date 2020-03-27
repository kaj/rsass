//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2260"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_2260/inner-parent-no-compound.hrx"
#[test]
fn inner_parent_no_compound() {
    assert_eq!(
        rsass(
            "@mixin test() {\r\
            \n  @at-root {\r\
            \n    .inner {\r\
            \n      @content;\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@include test {\r\
            \n  .test {\r\
            \n    property: value;\r\
            \n   }\r\
            \n }"
        )
        .unwrap(),
        ".inner .test {\
        \n  property: value;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2260/inner-parent-with-compound.hrx"
#[test]
fn inner_parent_with_compound() {
    assert_eq!(
        rsass(
            "@mixin test() {\r\
            \n  @at-root {\r\
            \n    .inner {\r\
            \n      @content;\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@include test {\r\
            \n  .test & {\r\
            \n    property: value;\r\
            \n   }\r\
            \n }"
        )
        .unwrap(),
        ".test .inner {\
        \n  property: value;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2260/no-parent-no-compound.hrx"
#[test]
fn no_parent_no_compound() {
    assert_eq!(
        rsass(
            "@mixin test() {\r\
            \n  @at-root {\r\
            \n    @content;\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@include test {\r\
            \n  .test {\r\
            \n    property: value;\r\
            \n   }\r\
            \n }"
        )
        .unwrap(),
        ".test {\
        \n  property: value;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2260/no-parent-with-compound.hrx"

// Ignoring "no_parent_with_compound", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_2260/outer-parent-no-compound.hrx"
#[test]
fn outer_parent_no_compound() {
    assert_eq!(
        rsass(
            "@mixin test() {\r\
            \n  .outer {\r\
            \n    @at-root {\r\
            \n      @content;\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@include test {\r\
            \n  .test {\r\
            \n    property: value;\r\
            \n   }\r\
            \n }"
        )
        .unwrap(),
        ".test {\
        \n  property: value;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2260/outer-parent-with-compound.hrx"
#[test]
fn outer_parent_with_compound() {
    assert_eq!(
        rsass(
            "@mixin test() {\r\
            \n  .outer {\r\
            \n    @at-root {\r\
            \n      @content;\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@include test {\r\
            \n  .test & {\r\
            \n    property: value;\r\
            \n   }\r\
            \n }"
        )
        .unwrap(),
        ".test .outer {\
        \n  property: value;\
        \n}\
        \n"
    );
}
