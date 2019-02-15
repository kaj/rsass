//! Tests auto-converted from "sass-spec/spec/selector-functions/extend"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/selector-functions/extend/nested.hrx"
#[test]
#[ignore] // failing
fn nested() {
    assert_eq!(
        rsass(
            ".qux {\n  &.waldo {\n    .where & {\n      .final {\n        f: selector-extend(#{&} foo, \'foo\', \'.bar\');\n      }\n    }\n  }\n}"
        )
        .unwrap(),
        ".where .qux.waldo .final {\n  f: .where .qux.waldo .final foo, .where .qux.waldo .final .bar;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/extend/simple.hrx"
#[test]
#[ignore] // failing
fn simple() {
    assert_eq!(
        rsass(
            ".simple {\n  a: selector-extend(\".a .b\", \".b\", \".foo .bar\");\n  b: selector-extend(\'.foo .x\', \'.x\', \'.a .bar\');\n  c: selector-extend(\'.foo .x, .x.bar\', \'.x\', \'.bang\');\n  d: selector-extend(\'.y .x\', \'.x, .y\', \'.foo\');\n  e: selector-extend(\'.foo .x\', \'.x\', \'.bar, .bang\');\n  f: selector-extend(\'.foo.x\', \'.x\', \'.foo\');\n}\n"
        )
        .unwrap(),
        ".simple {\n  a: .a .b, .a .foo .bar, .foo .a .bar;\n  b: .foo .x, .foo .a .bar, .a .foo .bar;\n  c: .foo .x, .foo .bang, .x.bar, .bar.bang;\n  d: .y .x, .foo .x, .y .foo, .foo .foo;\n  e: .foo .x, .foo .bar, .foo .bang;\n  f: .foo.x, .foo;\n}\n"
    );
}
