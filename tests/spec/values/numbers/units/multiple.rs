//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple.hrx"

#[test]
fn divide_by_multiple_denominators() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: inspect(1 / (1 / 1px / 1rad));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1px*rad;\
        \n}\
        \n"
    );
}
#[test]
fn divide_by_multiple_numerators() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: inspect(1 / (1px * 1rad));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1(px*rad)^-1;\
        \n}\
        \n"
    );
}
#[test]
fn division_cancels_both() {
    assert_eq!(
        crate::rsass(
            "$number: 1px * 1rad / 1ms / 1Hz;\
            \na {\
            \n  b: inspect($number / (1px / 1ms));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1rad/Hz;\
        \n}\
        \n"
    );
}
#[test]
fn division_cancels_compatible() {
    assert_eq!(
        crate::rsass(
            "$number: 1px * 1rad / 1ms / 1Hz;\
            \na {\
            \n  b: inspect($number / 1in);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.0104166667rad/ms*Hz;\
        \n}\
        \n"
    );
}
#[test]
fn division_cancels_denominator() {
    assert_eq!(
        crate::rsass(
            "$number: 1px * 1rad / 1ms / 1Hz;\
            \na {\
            \n  b: inspect($number / (1 / 1ms));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1px*rad/Hz;\
        \n}\
        \n"
    );
}
#[test]
fn division_cancels_denominator_twice() {
    assert_eq!(
        crate::rsass(
            "$number: 1px * 1rad / 1ms / 1Hz;\
            \na {\
            \n  b: inspect($number / (1 / 1ms / 1Hz));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1px*rad;\
        \n}\
        \n"
    );
}
#[test]
fn division_cancels_numerator() {
    assert_eq!(
        crate::rsass(
            "$number: 1px * 1rad / 1ms / 1Hz;\
            \na {\
            \n  b: inspect($number / 1px);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1rad/ms*Hz;\
        \n}\
        \n"
    );
}
#[test]
fn division_cancels_numerator_twice() {
    assert_eq!(
        crate::rsass(
            "$number: 1px * 1rad / 1ms / 1Hz;\
            \na {\
            \n  b: inspect($number / (1px * 1rad));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1(ms*Hz)^-1;\
        \n}\
        \n"
    );
}
#[test]
fn division_cancels_unknown() {
    assert_eq!(
        crate::rsass(
            "// Units cancel even if they\'re totally unknown to Sass.\
            \n$number: 1foo * 1bar / 1baz / 1qux;\
            \na {\
            \n  b: inspect($number / 1foo);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1bar/baz*qux;\
        \n}\
        \n"
    );
}
#[test]
fn multiple_denominators() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: inspect((1 / 1px / 1rad));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1(px*rad)^-1;\
        \n}\
        \n"
    );
}
#[test]
fn multiple_numerators() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: inspect(1px * 1rad);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1px*rad;\
        \n}\
        \n"
    );
}
#[test]
fn multiplication_cancels_both() {
    assert_eq!(
        crate::rsass(
            "$number: 1px * 1rad / 1ms / 1Hz;\
            \na {\
            \n  b: inspect($number * (1ms / 1px));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1rad/Hz;\
        \n}\
        \n"
    );
}
#[test]
fn multiplication_cancels_compatible() {
    assert_eq!(
        crate::rsass(
            "$number: 1px * 1rad / 1ms / 1Hz;\
            \na {\
            \n  b: inspect($number * 1s);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1000px*rad/Hz;\
        \n}\
        \n"
    );
}
#[test]
fn multiplication_cancels_denominator() {
    assert_eq!(
        crate::rsass(
            "$number: 1px * 1rad / 1ms / 1Hz;\
            \na {\
            \n  b: inspect($number * 1ms);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1px*rad/Hz;\
        \n}\
        \n"
    );
}
#[test]
fn multiplication_cancels_denominator_twice() {
    assert_eq!(
        crate::rsass(
            "$number: 1px * 1rad / 1ms / 1Hz;\
            \na {\
            \n  b: inspect($number * (1ms * 1Hz));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1px*rad;\
        \n}\
        \n"
    );
}
#[test]
fn multiplication_cancels_numerator() {
    assert_eq!(
        crate::rsass(
            "$number: 1px * 1rad / 1ms / 1Hz;\
            \na {\
            \n  b: inspect($number * (1 / 1px));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1rad/ms*Hz;\
        \n}\
        \n"
    );
}
#[test]
fn multiplication_cancels_numerator_twice() {
    assert_eq!(
        crate::rsass(
            "$number: 1px * 1rad / 1ms / 1Hz;\
            \na {\
            \n  b: inspect($number * (1 / 1px / 1rad));\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1(ms*Hz)^-1;\
        \n}\
        \n"
    );
}
#[test]
fn multiplication_cancels_unknown() {
    assert_eq!(
        crate::rsass(
            "// Units cancel even if they\'re totally unknown to Sass.\
            \n$number: 1foo * 1bar / 1baz / 1qux;\
            \na {\
            \n  b: inspect($number * 1baz);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1foo*bar/qux;\
        \n}\
        \n"
    );
}
