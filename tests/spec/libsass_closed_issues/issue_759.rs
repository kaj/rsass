//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_759.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$a: 10px !global !default;\
            \n$b: 20px !default !global;\
            \n$c: 30px !default !default !default !global !global !global;\
            \n$d: 40px !global !global !global !default !default !default;\
            \n$e: 50px !global !default !global !default !global !default;\
            \n\
            \nfoo {\
            \n  a: $a;\
            \n  b: $b;\
            \n  c: $c;\
            \n  d: $d;\
            \n  e: $e;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 10px;\
        \n  b: 20px;\
        \n  c: 30px;\
        \n  d: 40px;\
        \n  e: 50px;\
        \n}\
        \n"
    );
}
