//! These are from the `selector-functions` directory in the sass specification.
use rsass::compile_scss;

// Manual variant, slight todo from real test.
#[test]
fn parse() {
    check(
        "@mixin selector-info($selector) {\n  \
         parse: selector-parse($selector);\n}\n\n\
         .foo {\n  parse: selector-parse(&)\n}\n\n\
         #foo {\n  parse: selector-parse(&)\n}\n\n\
         .bar a {\n  parse: selector-parse(&);\n}\n\n\
         .bar,\n.baz {\n  parse: selector-parse(&)\n}\n\n\
         .qux {\n  &.waldo {\n    .where & {\n      \
         .final {\n        parse: selector-parse(&)\n      }\n    \
         }\n  }\n}\n\n\
         inside {\n  &.of {\n    #a {\n      .mixin{\n        \
         parse: selector-parse(&)\n      }\n    }\n  }\n}\n\n",
        // TODO a ".bar, .baz" here should actually be ".bar,\n.baz"
        ".foo {\n  parse: .foo;\n}\n\n\
         #foo {\n  parse: #foo;\n}\n\n\
         .bar a {\n  parse: .bar a;\n}\n\n\
         .bar, .baz {\n  parse: .bar, .baz;\n}\n\n\
         .where .qux.waldo .final {\n  parse: .where .qux.waldo .final;\n}\n\n\
         inside.of #a .mixin {\n  parse: inside.of #a .mixin;\n}\n",
    )
}

fn check(input: &str, expected: &str) {
    assert_eq!(
        String::from_utf8(
            compile_scss(input.as_ref(), Default::default()).unwrap()
        )
        .unwrap(),
        expected
    );
}
