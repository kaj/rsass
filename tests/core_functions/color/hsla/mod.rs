//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

mod error;

mod four_args;

/// From "sass-spec/spec/core_functions/color/hsla/multi_argument_var.hrx"
#[test]
fn multi_argument_var() {
    assert_eq!(
        rsass(
            "a {\n  // var() is substituted before parsing, so it may contain multiple arguments.\n  b: hsla(var(--foo), 3%, 0.4);\n  b: hsla(1, var(--foo), 0.4);\n  b: hsla(1, 2%, var(--foo));\n  b: hsla(var(--foo), 0.4);\n  b: hsla(1, var(--foo));\n  b: hsla(var(--foo));\n}\n"
        )
        .unwrap(),
        "a {\n  b: hsla(var(--foo), 3%, 0.4);\n  b: hsla(1, var(--foo), 0.4);\n  b: hsla(1, 2%, var(--foo));\n  b: hsla(var(--foo), 0.4);\n  b: hsla(1, var(--foo));\n  b: hsla(var(--foo));\n}\n"
    );
}

mod one_arg;

mod three_args;

// Ignoring "two_args.hrx", error tests are not supported yet.
