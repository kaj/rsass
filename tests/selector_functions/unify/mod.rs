//! Tests auto-converted from "sass-spec/spec/selector-functions/unify"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/selector-functions/unify/base"
#[test]
#[ignore] // failing
fn base() {
    assert_eq!(
        rsass(
            ".simple {\n  a: selector-unify(\'.foo\', \'.foo\');\n  b: selector-unify(\'.foo\', \'.bar\');\n  c: selector-unify(\'.foo.bar\', \'.bar.baz\');\n  d: selector-unify(\'.a .foo\', \'.b .bar\');\n  e: selector-unify(\'.a .foo\', \'.a .bar\');\n  f: selector-unify(\'p\', \'a\');\n  g: selector-unify(\'.foo >\', \'.bar\');\n  h: selector-unify(\'.foo\', \'.bar >\');\n  i: selector-unify(\'.foo, .bar\', \'.baz, .bang\');\n}\n\n.foo, .bar {\n  content: selector-unify(&, \'.baz, .bang\');\n}"
        )
        .unwrap(),
        ".simple {\n  a: .foo;\n  b: .foo.bar;\n  c: .foo.bar.baz;\n  d: .a .b .foo.bar, .b .a .foo.bar;\n  e: .a .foo.bar;\n  i: .foo.baz, .foo.bang, .bar.baz, .bar.bang;\n}\n.foo, .bar {\n  content: .foo.baz, .foo.bang, .bar.baz, .bar.bang;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/unify/universal_simple"
#[test]
#[ignore] // failing
fn universal_simple() {
    assert_eq!(
        rsass(
            "universal-selector {\r\n  test-01: selector-unify(\"*\", \"*\");\r\n  test-02: selector-unify(\"|*\", \"*\");\r\n  test-03: selector-unify(\"*|*\", \"*\");\r\n  test-04: selector-unify(\"*\", \"|*\");\r\n  test-05: selector-unify(\"|*\", \"|*\");\r\n  test-06: selector-unify(\"*|*\", \"|*\");\r\n  test-07: selector-unify(\"*\", \"*|*\");\r\n  test-08: selector-unify(\"|*\", \"*|*\");\r\n  test-09: selector-unify(\"*|*\", \"*|*\");\r\n}\r\n\r\ntag-selector {\r\n  test-1: selector-unify(\"tag\", \"*\");\r\n  test-2: selector-unify(\"tag\", \"|*\");\r\n  test-3: selector-unify(\"tag\", \"*|*\");\r\n  test-4: selector-unify(\"*\", \"tag\");\r\n  test-5: selector-unify(\"|*\", \"tag\");\r\n  test-6: selector-unify(\"*|*\", \"tag\");\r\n}\r\n\r\nclass-selector {\r\n  test-1: selector-unify(\".class\", \"*\");\r\n  test-2: selector-unify(\".class\", \"|*\");\r\n  test-3: selector-unify(\".class\", \"*|*\");\r\n  test-4: selector-unify(\"*\", \".class\");\r\n  test-5: selector-unify(\"|*\", \".class\");\r\n  test-6: selector-unify(\"*|*\", \".class\");\r\n}\r\n\r\nid-selector {\r\n  test-1: selector-unify(\"#id\", \"*\");\r\n  test-2: selector-unify(\"#id\", \"|*\");\r\n  test-3: selector-unify(\"#id\", \"*|*\");\r\n  test-4: selector-unify(\"*\", \"#id\");\r\n  test-5: selector-unify(\"|*\", \"#id\");\r\n  test-6: selector-unify(\"*|*\", \"#id\");\r\n}"
        )
        .unwrap(),
        "universal-selector {\n  test-01: *;\n  test-03: *;\n  test-05: |*;\n  test-06: |*;\n  test-07: *;\n  test-08: |*;\n  test-09: *|*;\n}\ntag-selector {\n  test-1: tag;\n  test-3: tag;\n  test-4: tag;\n  test-6: tag;\n}\nclass-selector {\n  test-1: .class;\n  test-2: |*.class;\n  test-3: .class;\n  test-4: .class;\n  test-5: |*.class;\n  test-6: .class;\n}\nid-selector {\n  test-1: #id;\n  test-2: |*#id;\n  test-3: #id;\n  test-4: #id;\n  test-5: |*#id;\n  test-6: #id;\n}\n"
    );
}
