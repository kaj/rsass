//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/arglists/can-end-with-comma/mixins.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixins")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin one-positional-arg($a,) {\
             \n  one-positional-arg: positional 1 $a;\
             \n}\n\
             \n@mixin two-positional-args($a, $b,) {\
             \n  two-positional-args: positional 2 $a $b;\
             \n}\n\
             \n@mixin one-keyword-arg($a: a,) {\
             \n  one-keyword-arg: keyword 1 $a;\
             \n}\n\
             \n@mixin two-keyword-args($a: a, $b: b,) {\
             \n  two-keyword-args: keyword 2 $a $b;\
             \n}\n\
             \n@mixin mixed-args($a, $b: b,) {\
             \n  mixed-args: keyword 2 $a $b;\
             \n}\n\
             \n.includes {\
             \n  @include one-positional-arg(a,);\
             \n  @include two-positional-args(a,b,);\
             \n  @include one-keyword-arg($a: z,);\
             \n  @include two-keyword-args($a: y,$b: z,);\
             \n  @include mixed-args(y, $b: z,);\
             \n}\n\n"),
        ".includes {\
         \n  one-positional-arg: positional 1 a;\
         \n  two-positional-args: positional 2 a b;\
         \n  one-keyword-arg: keyword 1 z;\
         \n  two-keyword-args: keyword 2 y z;\
         \n  mixed-args: keyword 2 y z;\
         \n}\n"
    );
}
