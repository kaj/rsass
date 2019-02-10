//! Tests auto-converted from "sass-spec/spec/core_functions/join"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

mod error;

/// From "sass-spec/spec/core_functions/join/valid"
#[test]
fn valid() {
    assert_eq!(
        rsass(
            "// 3.5 added the $bracketed parameter to join() and made it generally\n// bracketed-list-aware.\n.join {\n  both-bracketed: join([foo bar], [baz bang]);\n  first-bracketed: join([foo bar], baz bang);\n  second-bracketed: join(foo bar, [baz bang]);\n  bracketed-true: join(foo, bar, $bracketed: true);\n  bracketed-false: join([foo], [bar], $bracketed: false);\n  separator-and-bracketed: join(foo, bar, $separator: comma, $bracketed: true);\n  bracketed-and-separator: join(foo, bar, $bracketed: true, $separator: comma);\n  separator-and-bracketed-positional:\n      join(foo, bar, comma, true);\n\n  // All values are valid in boolean contexts.\n  unusual-bracketed-type: join(foo, bar, $bracketed: foo);\n  bracketed-null: join([foo], [bar], $bracketed: null);\n}"
        )
        .unwrap(),
        ".join {\n  both-bracketed: [foo bar baz bang];\n  first-bracketed: [foo bar baz bang];\n  second-bracketed: foo bar baz bang;\n  bracketed-true: [foo bar];\n  bracketed-false: foo bar;\n  separator-and-bracketed: [foo, bar];\n  bracketed-and-separator: [foo, bar];\n  separator-and-bracketed-positional: [foo, bar];\n  unusual-bracketed-type: [foo bar];\n  bracketed-null: foo bar;\n}\n"
    );
}
