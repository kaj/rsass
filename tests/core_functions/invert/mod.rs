//! Tests auto-converted from "sass-spec/spec/core_functions/invert"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/core_functions/invert/weight-parameter"
#[test]
fn weight_parameter() {
    assert_eq!(
        rsass(
            ".invert-with-weight {\n  zero-percent: invert(#edc, 0%);\n  ten-percent: invert(#edc, 10%);\n  keyword: invert(#edc, $weight: 10%);\n  one-hundred-percent: invert(#edc, 100%);\n}\n"
        )
        .unwrap(),
        ".invert-with-weight {\n  zero-percent: #eeddcc;\n  ten-percent: #d8cabd;\n  keyword: #d8cabd;\n  one-hundred-percent: #112233;\n}\n"
    );
}
