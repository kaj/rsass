//! These are from the `selector-functions` directory in the sass specification.
extern crate rsass;
use rsass::{compile_scss, OutputStyle};

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

fn check(input: &str, expected: &str) {
    assert_eq!(
        compile_scss(input.as_bytes(), OutputStyle::Normal)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        expected
    );
}
