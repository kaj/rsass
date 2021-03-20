//! Tests auto-converted from "sass-spec/spec/values/lists/brackets.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".bracketed-lists {\
            \n  empty: [];\
            \n  single: [foo];\
            \n  multiple: [foo, bar];\
            \n  nested: [[foo], [[bar, baz]]];\
            \n  space-separated: [foo bar baz];\
            \n  trailing-comma: [foo, bar,];\
            \n\
            \n  // List functions treat it like a list.\
            \n  nth-comma: nth([foo, bar], 2);\
            \n  nth-space: nth([foo bar], 2);\
            \n  comma-separator: list-separator([foo, bar]);\
            \n  space-separator: list-separator([foo bar]);\
            \n\
            \n  // List functions preserve bracketedness.\
            \n  set-nth: set-nth([foo, bar, baz], 2, qux);\
            \n  append: append([foo, bar], baz);\
            \n  append-with-separator: append([foo, bar], baz, $separator: space);\
            \n\
            \n  // Inspection produces valid input. This also verifies that nested lists of\
            \n  // various sorts are parsed properly.\
            \n  inspect-empty: inspect([]);\
            \n  inspect-simple: inspect([foo, bar]);\
            \n  inspect-nested-bracketed: inspect([[foo]]);\
            \n  inspect-nested-unbracketed: inspect([(foo bar)]);\
            \n  inspect-nested-unbracketed-comma: inspect([foo bar,]);\
            \n  inspect-nested-unbracketed-singleton: inspect([(foo,)]);\
            \n}\
            \n"
        )
        .unwrap(),
        ".bracketed-lists {\
        \n  empty: [];\
        \n  single: [foo];\
        \n  multiple: [foo, bar];\
        \n  nested: [[foo], [[bar, baz]]];\
        \n  space-separated: [foo bar baz];\
        \n  trailing-comma: [foo, bar];\
        \n  nth-comma: bar;\
        \n  nth-space: bar;\
        \n  comma-separator: comma;\
        \n  space-separator: space;\
        \n  set-nth: [foo, qux, baz];\
        \n  append: [foo, bar, baz];\
        \n  append-with-separator: [foo bar baz];\
        \n  inspect-empty: [];\
        \n  inspect-simple: [foo, bar];\
        \n  inspect-nested-bracketed: [[foo]];\
        \n  inspect-nested-unbracketed: [(foo bar)];\
        \n  inspect-nested-unbracketed-comma: [foo bar,];\
        \n  inspect-nested-unbracketed-singleton: [(foo,)];\
        \n}\
        \n"
    );
}
