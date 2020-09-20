//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_659"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_659/issue_659.hrx"
#[test]
fn issue_659() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/libsass-closed-issues/issue_659/sass-script.hrx"
#[test]
fn sass_script() {
    assert_eq!(
        rsass(
            "$foo: null;\
            \n\
            \n@mixin bar() {\
            \n   bar: $foo;\
            \n}\
            \n\
            \n@mixin baz() {\
            \n   baz: $foo !important;\
            \n}\
            \n\
            \nfoo {\
            \n  baz: $foo;\
            \n}\
            \n\
            \nbar {\
            \n  @include bar;\
            \n}\
            \n\
            \nbaz {\
            \n  @include baz;\
            \n}\
            \n"
        )
        .unwrap(),
        "baz {\
        \n  baz: !important;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_659/static.hrx"
#[test]
#[ignore] // wrong result
fn test_static() {
    assert_eq!(
        rsass(
            "\
            \n%bam { bam: null; }\
            \n\
            \n@mixin bar() {\
            \n   bar: null;\
            \n}\
            \n\
            \n@mixin baz() {\
            \n   baz: null !important;\
            \n}\
            \n\
            \nfoo {\
            \n  foo: null;\
            \n}\
            \n\
            \nbar {\
            \n  @include bar;\
            \n}\
            \n\
            \nbaz {\
            \n  @include baz;\
            \n}\
            \n\
            \nbam {\
            \n  @extend %bam;\
            \n}\
            \n"
        )
        .unwrap(),
        "baz {\
        \n  baz: !important;\
        \n}\
        \n"
    );
}
