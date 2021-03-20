//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/functions.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
