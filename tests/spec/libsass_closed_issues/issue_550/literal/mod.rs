//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_550/literal"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_550/literal/dimension.hrx"
#[test]
fn dimension() {
    assert_eq!(
        rsass(
            "#foo {\
            \n  i: 10.0001px;\
            \n  j: 10.01px;\
            \n  k: -10.0001px;\
            \n  l: -10.01px; }\
            \n\
            \n#foo {\
            \n  i: 0.0001px;\
            \n  j: 0.01px;\
            \n  k: -0.0001px;\
            \n  l: -0.01px; }\
            \n\
            \n#foo {\
            \n  i: .0001px;\
            \n  j: .01px;\
            \n  k: -.0001px;\
            \n  l: -.01px; }\
            \n"
        )
        .unwrap(),
        "#foo {\
        \n  i: 10.0001px;\
        \n  j: 10.01px;\
        \n  k: -10.0001px;\
        \n  l: -10.01px;\
        \n}\
        \n#foo {\
        \n  i: 0.0001px;\
        \n  j: 0.01px;\
        \n  k: -0.0001px;\
        \n  l: -0.01px;\
        \n}\
        \n#foo {\
        \n  i: 0.0001px;\
        \n  j: 0.01px;\
        \n  k: -0.0001px;\
        \n  l: -0.01px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_550/literal/number.hrx"
#[test]
fn number() {
    assert_eq!(
        rsass(
            "#foo {\
            \n  a: 10.0001;\
            \n  b: 10.01;\
            \n  c: -10.0001;\
            \n  d: -10.01; }\
            \n\
            \n#foo {\
            \n  a: 0.0001;\
            \n  b: 0.01;\
            \n  c: -0.0001;\
            \n  d: -0.01; }\
            \n\
            \n#foo {\
            \n  a: .0001;\
            \n  b: .01;\
            \n  c: -.0001;\
            \n  d: -.01; }\
            \n"
        )
        .unwrap(),
        "#foo {\
        \n  a: 10.0001;\
        \n  b: 10.01;\
        \n  c: -10.0001;\
        \n  d: -10.01;\
        \n}\
        \n#foo {\
        \n  a: 0.0001;\
        \n  b: 0.01;\
        \n  c: -0.0001;\
        \n  d: -0.01;\
        \n}\
        \n#foo {\
        \n  a: 0.0001;\
        \n  b: 0.01;\
        \n  c: -0.0001;\
        \n  d: -0.01;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_550/literal/percent.hrx"
#[test]
fn percent() {
    assert_eq!(
        rsass(
            "#foo {\
            \n  e: 10.0001%;\
            \n  f: 10.01%;\
            \n  g: -10.0001%;\
            \n  h: -10.01%; }\
            \n\
            \n#foo {\
            \n  e: 0.0001%;\
            \n  f: 0.01%;\
            \n  g: -0.0001%;\
            \n  h: -0.01%; }\
            \n\
            \n#foo {\
            \n  e: .0001%;\
            \n  f: .01%;\
            \n  g: -.0001%;\
            \n  h: -.01%; }\
            \n"
        )
        .unwrap(),
        "#foo {\
        \n  e: 10.0001%;\
        \n  f: 10.01%;\
        \n  g: -10.0001%;\
        \n  h: -10.01%;\
        \n}\
        \n#foo {\
        \n  e: 0.0001%;\
        \n  f: 0.01%;\
        \n  g: -0.0001%;\
        \n  h: -0.01%;\
        \n}\
        \n#foo {\
        \n  e: 0.0001%;\
        \n  f: 0.01%;\
        \n  g: -0.0001%;\
        \n  h: -0.01%;\
        \n}\
        \n"
    );
}
