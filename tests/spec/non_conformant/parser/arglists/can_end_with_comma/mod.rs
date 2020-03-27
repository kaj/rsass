//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-call-1.hrx"

// Ignoring "error_call_1", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-call-2.hrx"

// Ignoring "error_call_2", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-call-3.hrx"

// Ignoring "error_call_3", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-function-1.hrx"

// Ignoring "error_function_1", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-function-2.hrx"

// Ignoring "error_function_2", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-function-3.hrx"

// Ignoring "error_function_3", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-include-1.hrx"

// Ignoring "error_include_1", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-include-2.hrx"

// Ignoring "error_include_2", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-include-3.hrx"

// Ignoring "error_include_3", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-mixin-1.hrx"

// Ignoring "error_mixin_1", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-mixin-2.hrx"

// Ignoring "error_mixin_2", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/error-mixin-3.hrx"

// Ignoring "error_mixin_3", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/functions.hrx"
#[test]
fn functions() {
    assert_eq!(
        rsass(
            "@function one-positional-arg($a,) {\
            \n  @return positional 1 $a;\
            \n}\
            \n\
            \n@function two-positional-args($a, $b,) {\
            \n  @return positional 2 $a $b;\
            \n}\
            \n\
            \n@function one-keyword-arg($a: a,) {\
            \n  @return keyword 1 $a;\
            \n}\
            \n\
            \n@function two-keyword-args($a: a, $b: b,) {\
            \n  @return keyword 2 $a $b;\
            \n}\
            \n\
            \n@function mixed-args($a, $b: b,) {\
            \n  @return keyword 2 $a $b;\
            \n}\
            \n\
            \n.calls {\
            \n  one-positional-arg: one-positional-arg(a,);\
            \n  two-positional-args: two-positional-args(a,b,);\
            \n  one-keyword-arg: one-keyword-arg($a: z,);\
            \n  two-keyword-args: two-keyword-args($a: y,$b: z,);\
            \n  mixed-args: mixed-args(y, $b: z,);\
            \n}\
            \n"
        )
        .unwrap(),
        ".calls {\
        \n  one-positional-arg: positional 1 a;\
        \n  two-positional-args: positional 2 a b;\
        \n  one-keyword-arg: keyword 1 z;\
        \n  two-keyword-args: keyword 2 y z;\
        \n  mixed-args: keyword 2 y z;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/mixins.hrx"
#[test]
fn mixins() {
    assert_eq!(
        rsass(
            "@mixin one-positional-arg($a,) {\
            \n  one-positional-arg: positional 1 $a;\
            \n}\
            \n\
            \n@mixin two-positional-args($a, $b,) {\
            \n  two-positional-args: positional 2 $a $b;\
            \n}\
            \n\
            \n@mixin one-keyword-arg($a: a,) {\
            \n  one-keyword-arg: keyword 1 $a;\
            \n}\
            \n\
            \n@mixin two-keyword-args($a: a, $b: b,) {\
            \n  two-keyword-args: keyword 2 $a $b;\
            \n}\
            \n\
            \n@mixin mixed-args($a, $b: b,) {\
            \n  mixed-args: keyword 2 $a $b;\
            \n}\
            \n\
            \n.includes {\
            \n  @include one-positional-arg(a,);\
            \n  @include two-positional-args(a,b,);\
            \n  @include one-keyword-arg($a: z,);\
            \n  @include two-keyword-args($a: y,$b: z,);\
            \n  @include mixed-args(y, $b: z,);\
            \n}\
            \n\
            \n"
        )
        .unwrap(),
        ".includes {\
        \n  one-positional-arg: positional 1 a;\
        \n  two-positional-args: positional 2 a b;\
        \n  one-keyword-arg: keyword 1 z;\
        \n  two-keyword-args: keyword 2 y z;\
        \n  mixed-args: keyword 2 y z;\
        \n}\
        \n"
    );
}
