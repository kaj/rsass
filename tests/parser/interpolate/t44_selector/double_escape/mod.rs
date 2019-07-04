//! Tests auto-converted from "sass-spec/spec/parser/interpolate/44_selector/double_escape"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/parser/interpolate/44_selector/double_escape/12_double_escaped_interpolated_value_todo.hrx"
#[test]
#[ignore] // failing
fn t12_double_escaped_interpolated_value_todo() {
    assert_eq!(
        rsass(
            "$key: \'bar\';\n.test12#{\'\\\\@#{$key}\'} { content: \'1.2\'; }\n"
        )
        .unwrap(),
        ".test12\\@bar {\n  content: \'1.2\';\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/44_selector/double_escape/22_double_escaped_interpolated_variable.hrx"
#[test]
#[ignore] // failing
fn t22_double_escaped_interpolated_variable() {
    assert_eq!(
        rsass(
            "$key: \'bar\';\n$suffix2: \'\\\\@#{$key}\';\n.test22#{$suffix2} { content: \'2.2\'; }\n"
        )
        .unwrap(),
        ".test22\\@bar {\n  content: \'2.2\';\n}\n"
    );
}

// From "sass-spec/spec/parser/interpolate/44_selector/double_escape/32_double_escaped_literal.hrx"
#[test]
#[ignore] // failing
fn t32_double_escaped_literal() {
    assert_eq!(
        rsass(".test32#{\'\\\\@baz\'} { content: \'3.2\'; }\n").unwrap(),
        ".test32\\@baz {\n  content: \'3.2\';\n}\n"
    );
}
