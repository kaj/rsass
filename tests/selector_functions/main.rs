//! Tests auto-converted from "sass-spec/spec/selector-functions"
//! version 6c9bd98c, 2019-02-13 14:56:19 -0800.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["is_superselector"]
use rsass::{compile_scss, OutputStyle};

/// From "sass-spec/spec/selector-functions/append.hrx"
#[test]
fn append() {
    assert_eq!(
        rsass(
            "body {\n  a: selector-append(\".foo\", \".bar\", \".baz\");\n  b: selector-append(\".a .foo\", \".b .bar\");\n  c: selector-append(\".foo\", \"-suffix\");\n  d: selector-append(\'.foo\', \'.bar, -suffix\');\n  e: selector-append(\'.foo\', \'--suffix\');\n  f: selector-append(\'.foo\', \'.bar, --suffix\');\n}"
        )
        .unwrap(),
        "body {\n  a: .foo.bar.baz;\n  b: .a .foo.b .bar;\n  c: .foo-suffix;\n  d: .foo.bar, .foo-suffix;\n  e: .foo--suffix;\n  f: .foo.bar, .foo--suffix;\n}\n"
    );
}

mod extend;

// Ignoring "is_superselector", not expected to work yet.

/// From "sass-spec/spec/selector-functions/nest.hrx"
#[test]
fn nest() {
    assert_eq!(
        rsass(
            ".simple {\n  a: selector-nest(\'.foo\');\n  b: selector-nest(\'.foo\', \'.bar\');\n  c: selector-nest(\'.foo\', \'.bar\', \'.baz\');\n  d: selector-nest(\'.a .foo\', \'.b .bar\');\n  e: selector-nest(\'.foo\', \'&.bar\');\n  e: selector-nest(\'.foo\', \'&.bar\', \'.baz &\');\n}"
        )
        .unwrap(),
        ".simple {\n  a: .foo;\n  b: .foo .bar;\n  c: .foo .bar .baz;\n  d: .a .foo .b .bar;\n  e: .foo.bar;\n  e: .baz .foo.bar;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/parse.hrx"
#[test]
#[ignore] // failing
fn parse() {
    assert_eq!(
        rsass(
            "@mixin selector-info($selector) {\n  parse: selector-parse($selector);\n}\n\n.foo {\n  parse: selector-parse(&)\n}\n\n#foo {\n  parse: selector-parse(&)\n}\n\n.bar a {\n  parse: selector-parse(&);\n}\n\n.bar,\n.baz {\n  parse: selector-parse(&)\n}\n\n.qux {\n  &.waldo {\n    .where & {\n      .final {\n        parse: selector-parse(&)\n      }\n    }\n  }\n}\n\ninside {\n  &.of {\n    #a {\n      .mixin{\n        parse: selector-parse(&)\n      }\n    }\n  }\n}\n\n"
        )
        .unwrap(),
        ".foo {\n  parse: .foo;\n}\n#foo {\n  parse: #foo;\n}\n.bar a {\n  parse: .bar a;\n}\n.bar,\n.baz {\n  parse: .bar, .baz;\n}\n.where .qux.waldo .final {\n  parse: .where .qux.waldo .final;\n}\ninside.of #a .mixin {\n  parse: inside.of #a .mixin;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/replace.hrx"
#[test]
#[ignore] // failing
fn replace() {
    assert_eq!(
        rsass(
            ".simple {\n  a: selector-replace(\'.foo\', \'.foo\', \'.bar\');\n  b: selector-replace(\'.foo.bar\', \'.bar\', \'.baz\');\n  c: selector-replace(\'.foo.bar\', \'.bar\', \'.a .baz\');\n  d: selector-replace(\'.foo.bar\', \'.baz.bar\', \'.qux\');\n  e: selector-replace(\'.foo.bar.baz\', \'.foo.baz\', \'.qux\');\n}"
        )
        .unwrap(),
        ".simple {\n  a: .bar;\n  b: .foo.baz;\n  c: .a .foo.baz;\n  d: .foo.bar;\n  e: .bar.qux;\n}\n"
    );
}

mod unify;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
