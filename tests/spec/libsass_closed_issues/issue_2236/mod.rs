//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2236"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-closed-issues/issue_2236/floats.hrx"
#[test]
#[ignore] // wrong result
fn floats() {
    assert_eq!(
        rsass(
            ".foo {\r\
            \n  test-01: +1.2 % +4.7;\r\
            \n  test-02: +1.2 % -4.7;\r\
            \n  test-03: -1.2 % +4.7;\r\
            \n  test-04: -1.2 % -4.7;\r\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  test-01: 1.2;\
        \n  test-02: -3.5;\
        \n  test-03: 3.5;\
        \n  test-04: -1.2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2236/ints.hrx"
#[test]
#[ignore] // wrong result
fn ints() {
    assert_eq!(
        rsass(
            ".foo {\r\
            \n  test-01: +1 % +4;\r\
            \n  test-02: +1 % -4;\r\
            \n  test-03: -1 % +4;\r\
            \n  test-04: -1 % -4;\r\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  test-01: 1;\
        \n  test-02: -3;\
        \n  test-03: 3;\
        \n  test-04: -1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2236/zeros.hrx"
#[test]
fn zeros() {
    assert_eq!(
        rsass(
            ".foo {\r\
            \n  test-01: +0 % 1;\r\
            \n  test-02: -0 % 1;\r\
            \n  test-03: +0 % -1;\r\
            \n  test-04: -0 % -1;\r\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  test-01: 0;\
        \n  test-02: 0;\
        \n  test-03: 0;\
        \n  test-04: 0;\
        \n}\
        \n"
    );
}
