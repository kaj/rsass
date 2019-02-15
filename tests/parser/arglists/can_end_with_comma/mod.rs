//! Tests auto-converted from "sass-spec/spec/parser/arglists/can-end-with-comma"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "error-call-1.hrx", error tests are not supported yet.

// Ignoring "error-call-2.hrx", error tests are not supported yet.

// Ignoring "error-call-3.hrx", error tests are not supported yet.

// Ignoring "error-function-1.hrx", error tests are not supported yet.

// Ignoring "error-function-2.hrx", error tests are not supported yet.

// Ignoring "error-function-3.hrx", error tests are not supported yet.

// Ignoring "error-include-1.hrx", error tests are not supported yet.

// Ignoring "error-include-2.hrx", error tests are not supported yet.

// Ignoring "error-include-3.hrx", error tests are not supported yet.

// Ignoring "error-mixin-1.hrx", error tests are not supported yet.

// Ignoring "error-mixin-2.hrx", error tests are not supported yet.

// Ignoring "error-mixin-3.hrx", error tests are not supported yet.

/// From "sass-spec/spec/parser/arglists/can-end-with-comma/functions.hrx"
#[test]
fn functions() {
    assert_eq!(
        rsass(
            "@function one-positional-arg($a,) {\n  @return positional 1 $a;\n}\n\n@function two-positional-args($a, $b,) {\n  @return positional 2 $a $b;\n}\n\n@function one-keyword-arg($a: a,) {\n  @return keyword 1 $a;\n}\n\n@function two-keyword-args($a: a, $b: b,) {\n  @return keyword 2 $a $b;\n}\n\n@function mixed-args($a, $b: b,) {\n  @return keyword 2 $a $b;\n}\n\n.calls {\n  one-positional-arg: one-positional-arg(a,);\n  two-positional-args: two-positional-args(a,b,);\n  one-keyword-arg: one-keyword-arg($a: z,);\n  two-keyword-args: two-keyword-args($a: y,$b: z,);\n  mixed-args: mixed-args(y, $b: z,);\n}\n"
        )
        .unwrap(),
        ".calls {\n  one-positional-arg: positional 1 a;\n  two-positional-args: positional 2 a b;\n  one-keyword-arg: keyword 1 z;\n  two-keyword-args: keyword 2 y z;\n  mixed-args: keyword 2 y z;\n}\n"
    );
}

/// From "sass-spec/spec/parser/arglists/can-end-with-comma/mixins.hrx"
#[test]
fn mixins() {
    assert_eq!(
        rsass(
            "@mixin one-positional-arg($a,) {\n  one-positional-arg: positional 1 $a;\n}\n\n@mixin two-positional-args($a, $b,) {\n  two-positional-args: positional 2 $a $b;\n}\n\n@mixin one-keyword-arg($a: a,) {\n  one-keyword-arg: keyword 1 $a;\n}\n\n@mixin two-keyword-args($a: a, $b: b,) {\n  two-keyword-args: keyword 2 $a $b;\n}\n\n@mixin mixed-args($a, $b: b,) {\n  mixed-args: keyword 2 $a $b;\n}\n\n.includes {\n  @include one-positional-arg(a,);\n  @include two-positional-args(a,b,);\n  @include one-keyword-arg($a: z,);\n  @include two-keyword-args($a: y,$b: z,);\n  @include mixed-args(y, $b: z,);\n}\n\n"
        )
        .unwrap(),
        ".includes {\n  one-positional-arg: positional 1 a;\n  two-positional-args: positional 2 a b;\n  one-keyword-arg: keyword 1 z;\n  two-keyword-args: keyword 2 y z;\n  mixed-args: keyword 2 y z;\n}\n"
    );
}
