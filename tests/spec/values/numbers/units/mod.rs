//! Tests auto-converted from "sass-spec/spec/values/numbers/units"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/values/numbers/units/multiple.hrx"
#[test]
#[ignore] // wrong result
fn multiple() {
    assert_eq!(
        rsass(
            ".multiple {\
            \n  // Sass units that are multiplied and divided along with the values.\
            \n  multiple-numerators: inspect(1px * 1rad);\
            \n  multiple-denominators: inspect((1 / 1px / 1rad));\
            \n  divide-by-multiple-numerators: inspect(1 / (1px * 1rad));\
            \n  divide-by-multiple-denominators: inspect(1 / (1 / 1px / 1rad));\
            \n\
            \n  // Units that appear in both the numerator and denominator cancel out, leaving\
            \n  // remaining units in place.\
            \n  $number: 1px * 1rad / 1ms / 1Hz;\
            \n  multiplication-cancels-denominator: inspect($number * 1ms);\
            \n  multiplication-cancels-denominator-twice: inspect($number * (1ms * 1Hz));\
            \n  multiplication-cancels-numerator: inspect($number * (1 / 1px));\
            \n  multiplication-cancels-numerator-twice: inspect($number * (1 / 1px / 1rad));\
            \n  multiplication-cancels-both: inspect($number * (1ms / 1px));\
            \n  division-cancels-numerator: inspect($number / 1rad);\
            \n  division-cancels-numerator-twice: inspect($number / (1px * 1rad));\
            \n  division-cancels-denominator: inspect($number / (1 / 1ms));\
            \n  division-cancels-denominator-twice: inspect($number / (1 / 1ms / 1Hz));\
            \n  division-cancels-both: inspect($number / (1px / 1ms));\
            \n\
            \n  // Units cancel if they\'re compatible, even if they aren\'t identical.\
            \n  multiplication-cancels-compatible: inspect($number * 1s);\
            \n  division-cancels-compatible: inspect($number / 1in);\
            \n\
            \n  // Units cancel even if they\'re totally unknown to Sass.\
            \n  $number: 1foo * 1bar / 1baz / 1qux;\
            \n  multiplication-cancels-unknown: inspect($number * 1baz);\
            \n  division-cancels-unknown: inspect($number / 1foo);\
            \n}\
            \n"
        )
        .unwrap(),
        ".multiple {\
        \n  multiple-numerators: 1px*rad;\
        \n  multiple-denominators: 1(px*rad)^-1;\
        \n  divide-by-multiple-numerators: 1(px*rad)^-1;\
        \n  divide-by-multiple-denominators: 1px*rad;\
        \n  multiplication-cancels-denominator: 1px*rad/Hz;\
        \n  multiplication-cancels-denominator-twice: 1px*rad;\
        \n  multiplication-cancels-numerator: 1rad/ms*Hz;\
        \n  multiplication-cancels-numerator-twice: 1(ms*Hz)^-1;\
        \n  multiplication-cancels-both: 1rad/Hz;\
        \n  division-cancels-numerator: 1px/ms*Hz;\
        \n  division-cancels-numerator-twice: 1(ms*Hz)^-1;\
        \n  division-cancels-denominator: 1px*rad/Hz;\
        \n  division-cancels-denominator-twice: 1px*rad;\
        \n  division-cancels-both: 1rad/Hz;\
        \n  multiplication-cancels-compatible: 1000px*rad/Hz;\
        \n  division-cancels-compatible: 0.0104166667rad/ms*Hz;\
        \n  multiplication-cancels-unknown: 1foo*bar/qux;\
        \n  division-cancels-unknown: 1bar/baz*qux;\
        \n}\
        \n"
    );
}
