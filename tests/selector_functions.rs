//! These are from the `selector-functions` directory in the sass specification.
extern crate rsass;
use rsass::{compile_scss, OutputStyle};

#[test]
fn append() {
    check(
        "body {\n  a: selector-append(\".foo\", \".bar\", \".baz\");\n  \
         b: selector-append(\".a .foo\", \".b .bar\");\n  \
         c: selector-append(\".foo\", \"-suffix\");\n  \
         d: selector-append('.foo', '.bar, -suffix');\n  \
         e: selector-append('.foo', '--suffix');\n  \
         f: selector-append('.foo', '.bar, --suffix');\n}",
        "body {\n  a: .foo.bar.baz;\n  b: .a .foo.b .bar;\n  \
         c: .foo-suffix;\n  d: .foo.bar, .foo-suffix;\n  e: .foo--suffix;\n  \
         f: .foo.bar, .foo--suffix;\n}\n",
    )
}

#[test]
fn nest() {
    check(
        ".simple {\n  a: selector-nest('.foo');\n  \
         b: selector-nest('.foo', '.bar');\n  \
         c: selector-nest('.foo', '.bar', '.baz');\n  \
         d: selector-nest('.a .foo', '.b .bar');\n  \
         e: selector-nest('.foo', '&.bar');\n  \
         e: selector-nest('.foo', '&.bar', '.baz &');\n}",
        ".simple {\n  a: .foo;\n  b: .foo .bar;\n  c: .foo .bar .baz;\n  \
         d: .a .foo .b .bar;\n  e: .foo.bar;\n  e: .baz .foo.bar;\n}\n",
    )
}

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
        compile_scss(input.as_bytes(), OutputStyle::Expanded)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        expected
    );
}
