//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_550"
#[allow(unused)]
use super::rsass;

mod literal;

// From "sass-spec/spec/libsass-closed-issues/issue_550/operator.hrx"
#[test]
fn operator() {
    assert_eq!(
        rsass(
            "$x: .03;\
            \n$y: 0.7;\
            \n\
            \n#foo {\
            \n  color: saturate($x);\
            \n  color: saturate($y);\
            \n  -webkit-filter: grayscale(.03);\
            \n  -webkit-filter: grayscale(0.7);\
            \n}\
            \n"
        )
        .unwrap(),
        "#foo {\
        \n  color: saturate(0.03);\
        \n  color: saturate(0.7);\
        \n  -webkit-filter: grayscale(0.03);\
        \n  -webkit-filter: grayscale(0.7);\
        \n}\
        \n"
    );
}

mod variable;
