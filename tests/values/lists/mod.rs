//! Tests auto-converted from "sass-spec/spec/values/lists"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/values/lists/brackets"
#[test]
fn brackets() {
    assert_eq!(
        rsass(
            ".bracketed-lists {\n  empty: [];\n  single: [foo];\n  multiple: [foo, bar];\n  nested: [[foo], [[bar, baz]]];\n  space-separated: [foo bar baz];\n  trailing-comma: [foo, bar,];\n\n  // List functions treat it like a list.\n  nth-comma: nth([foo, bar], 2);\n  nth-space: nth([foo bar], 2);\n  comma-separator: list-separator([foo, bar]);\n  space-separator: list-separator([foo bar]);\n\n  // List functions preserve bracketedness.\n  set-nth: set-nth([foo, bar, baz], 2, qux);\n  append: append([foo, bar], baz);\n  append-with-separator: append([foo, bar], baz, $separator: space);\n\n  // Inspection produces valid input. This also verifies that nested lists of\n  // various sorts are parsed properly.\n  inspect-empty: inspect([]);\n  inspect-simple: inspect([foo, bar]);\n  inspect-nested-bracketed: inspect([[foo]]);\n  inspect-nested-unbracketed: inspect([(foo bar)]);\n  inspect-nested-unbracketed-comma: inspect([foo bar,]);\n  inspect-nested-unbracketed-singleton: inspect([(foo,)]);\n}\n"
        )
        .unwrap(),
        ".bracketed-lists {\n  empty: [];\n  single: [foo];\n  multiple: [foo, bar];\n  nested: [[foo], [[bar, baz]]];\n  space-separated: [foo bar baz];\n  trailing-comma: [foo, bar];\n  nth-comma: bar;\n  nth-space: bar;\n  comma-separator: comma;\n  space-separator: space;\n  set-nth: [foo, qux, baz];\n  append: [foo, bar, baz];\n  append-with-separator: [foo bar baz];\n  inspect-empty: [];\n  inspect-simple: [foo, bar];\n  inspect-nested-bracketed: [[foo]];\n  inspect-nested-unbracketed: [(foo bar)];\n  inspect-nested-unbracketed-comma: [foo bar,];\n  inspect-nested-unbracketed-singleton: [(foo,)];\n}\n"
    );
}

/// From "sass-spec/spec/values/lists/equality"
#[test]
fn equality() {
    assert_eq!(
        rsass(
            "a {\n  @if [foo bar]==[foo bar] {\n    t1: t;\n  } @else {\n    f1: f;\n  }\n\n  @if [foo bar]==[foo, bar] {\n    t2: t;\n  } @else {\n    f2: f;\n  }\n\n  @if [foo bar]==(foo bar) {\n    t3: t;\n  } @else {\n    f3: f;\n  }\n\n  @if [] == [] {\n    t4: t;\n  } @else {\n    f4: f;\n  }\n\n  @if [] == () {\n    t5: t;\n  } @else {\n    f5: f;\n  }\n}\n"
        )
        .unwrap(),
        "a {\n  t1: t;\n  f2: f;\n  f3: f;\n  t4: t;\n  f5: f;\n}\n"
    );
}
