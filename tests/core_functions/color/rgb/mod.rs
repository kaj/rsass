//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

mod error;

mod four_args;

/// From "sass-spec/spec/core_functions/color/rgb/multi_argument_var.hrx"
#[test]
fn multi_argument_var() {
    assert_eq!(
        rsass(
            "a {\n  // var() is substituted before parsing, so it may contain multiple arguments.\n  b: rgb(var(--foo), 3, 0.4);\n  b: rgb(1, var(--foo), 0.4);\n  b: rgb(1, 2, var(--foo));\n  b: rgb(var(--foo), 0.4);\n  b: rgb(1, var(--foo));\n  b: rgb(var(--foo));\n}\n"
        )
        .unwrap(),
        "a {\n  b: rgb(var(--foo), 3, 0.4);\n  b: rgb(1, var(--foo), 0.4);\n  b: rgb(1, 2, var(--foo));\n  b: rgb(var(--foo), 0.4);\n  b: rgb(1, var(--foo));\n  b: rgb(var(--foo));\n}\n"
    );
}

mod one_arg;

mod three_args;

/// From "sass-spec/spec/core_functions/color/rgb/two_args.hrx"
#[test]
#[ignore] // failing
fn two_args() {
    assert_eq!(
        rsass(
            "opaque-to {\n  opaque: rgb(#123, 1);\n  partial: rgb(#123, 0.5);\n  transparent: rgb(#123, 0);\n}\n\npartial-to {\n  $color: rgb(0, 0, 255, 0.3);\n  opaque: rgb($color, 1);\n  partial: rgb($color, 0.5);\n  transparent: rgb($color, 0);\n}\n\ntransparent-to {\n  opaque: rgb(transparent, 1);\n  partial: rgb(transparent, 0.5);\n  transparent: rgb(transparent, 0);\n}\n\nclamped {\n  opaque: rgb(#123, 1.1);\n  transparent: rgb(#123, -0.1);\n}\n\nnamed {\n  x: rgb($color: #123, $alpha: 0.5);\n}\n"
        )
        .unwrap(),
        "opaque-to {\n  opaque: #112233;\n  partial: rgba(17, 34, 51, 0.5);\n  transparent: rgba(17, 34, 51, 0);\n}\npartial-to {\n  opaque: blue;\n  partial: rgba(0, 0, 255, 0.5);\n  transparent: rgba(0, 0, 255, 0);\n}\ntransparent-to {\n  opaque: black;\n  partial: rgba(0, 0, 0, 0.5);\n  transparent: rgba(0, 0, 0, 0);\n}\nclamped {\n  opaque: #112233;\n  transparent: rgba(17, 34, 51, 0);\n}\nnamed {\n  x: rgba(17, 34, 51, 0.5);\n}\n"
    );
}
