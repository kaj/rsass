//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/fake-pseudo-element-order"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/extend-tests/fake-pseudo-element-order/after.hrx"
#[test]
#[ignore] // unexepected error
fn after() {
    assert_eq!(
        rsass(
            "%a:after {x: y}\
            \nb:c {@extend %a}\
            \n"
        )
        .unwrap(),
        "b:c:after {\
        \n  x: y;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/fake-pseudo-element-order/before.hrx"
#[test]
#[ignore] // unexepected error
fn before() {
    assert_eq!(
        rsass(
            "%a:before {x: y}\
            \nb:c {@extend %a}\
            \n"
        )
        .unwrap(),
        "b:c:before {\
        \n  x: y;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/fake-pseudo-element-order/first-letter.hrx"
#[test]
#[ignore] // unexepected error
fn first_letter() {
    assert_eq!(
        rsass(
            "%a:first-letter {x: y}\
            \nb:c {@extend %a}\
            \n"
        )
        .unwrap(),
        "b:c:first-letter {\
        \n  x: y;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/extend-tests/fake-pseudo-element-order/first-line.hrx"
#[test]
#[ignore] // unexepected error
fn first_line() {
    assert_eq!(
        rsass(
            "%a:first-line {x: y}\
            \nb:c {@extend %a}\
            \n"
        )
        .unwrap(),
        "b:c:first-line {\
        \n  x: y;\
        \n}\
        \n"
    );
}
