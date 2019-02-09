//! Tests auto-converted from "sass-spec/spec/parser/operations"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

mod addition;

/// From "sass-spec/spec/parser/operations/binary-and-unary"
#[test]
#[ignore] // failing
fn binary_and_unary() {
    assert_eq!(
        rsass(
            "foo {\n  minus-before-minus: - 1 - 2;\n  minus-after-minus:  1 - - 2;\n  plus-before-minus:  + 1 - 2;\n  plus-after-minus:   1 - + 2;\n  not-before-plus:    not 1 + 2;\n  not-after-plus:     1 + not 2;\n\n  minus-after-comma:  (1, - 2);\n  plus-after-comma:   (1, + 2);\n  slash-after-comma:  (1, / 2);\n  not-after-comma:    (1, not 2);\n}\n"
        )
        .unwrap(),
        "foo {\n  minus-before-minus: -3;\n  minus-after-minus: 3;\n  plus-before-minus: -1;\n  plus-after-minus: -1;\n  not-before-plus: false2;\n  not-after-plus: 1false;\n  minus-after-comma: 1, -2;\n  plus-after-comma: 1, 2;\n  slash-after-comma: 1, /2;\n  not-after-comma: 1, false;\n}\n"
    );
}

mod division;

mod logic_eq;

mod logic_ge;

mod logic_gt;

mod logic_le;

mod logic_lt;

mod logic_ne;

mod modulo;

mod multiply;

mod subtract;
