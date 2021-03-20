//! Tests auto-converted from "sass-spec/spec/non_conformant/sass/var-args/success.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($x, $y, $zs...) {\
            \n  grarh: type-of(false);\
            \n  f: $zs;\
            \n  fa: $x, $y, $zs;\
            \n  fv: $zs;\
            \n  ft: type-of($zs);\
            \n  fj: join(1 2 3, $zs);\
            \n  fjt: type-of(join(1 2 3, $zs));\
            \n  fkt: type-of(join($zs, 1 2 3));\
            \n  hoopla: length(a b c d e);\
            \n  boopla: type-of(123+234);\
            \n  koopla: type-of(length(a b c d));\
            \n}\
            \n\
            \n@mixin bar($x, $y, $z) {\
            \n  ba: $x, $y, $z;\
            \n  bv: $z;\
            \n  bt: type-of($z);\
            \n  bj: join(1 2 3, $z);\
            \n  bjt: type-of(join(1 2 3, $z));\
            \n}\
            \n\
            \n$stuff: hey hoo ha;\
            \n\
            \n@mixin mudge($x, $y, $zs...) {\
            \n  x: $x;\
            \n  y: $y;\
            \n  z: $zs;\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo(a, b, c d e f);\
            \n  @include bar(a, b, c d e f);\
            \n  @include foo(a, b, c d e...);\
            \n  @include foo(a, c d e...);\
            \n  @include foo(a, $stuff...);\
            \n  new: type-of(\"hello\");\
            \n}\
            \n\
            \n@mixin bad($x, $y, $z) {\
            \n  first: $x;\
            \n  second: $y;\
            \n  rest: $z;\
            \n}\
            \n\
            \n@mixin foo($x, $y, $z) {\
            \n  a: type-of(join($x, $y));\
            \n  b: type-of($z);\
            \n  c: type-of(length($x));\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo(a b c, d e, false);\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  grarh: bool;\
        \n  f: c d e f;\
        \n  fa: a, b, c d e f;\
        \n  fv: c d e f;\
        \n  ft: arglist;\
        \n  fj: 1 2 3 c d e f;\
        \n  fjt: list;\
        \n  fkt: list;\
        \n  hoopla: 5;\
        \n  boopla: number;\
        \n  koopla: number;\
        \n  ba: a, b, c d e f;\
        \n  bv: c d e f;\
        \n  bt: list;\
        \n  bj: 1 2 3 c d e f;\
        \n  bjt: list;\
        \n  grarh: bool;\
        \n  f: c d e;\
        \n  fa: a, b, c d e;\
        \n  fv: c d e;\
        \n  ft: arglist;\
        \n  fj: 1 2 3 c d e;\
        \n  fjt: list;\
        \n  fkt: list;\
        \n  hoopla: 5;\
        \n  boopla: number;\
        \n  koopla: number;\
        \n  grarh: bool;\
        \n  f: d e;\
        \n  fa: a, c, d e;\
        \n  fv: d e;\
        \n  ft: arglist;\
        \n  fj: 1 2 3 d e;\
        \n  fjt: list;\
        \n  fkt: list;\
        \n  hoopla: 5;\
        \n  boopla: number;\
        \n  koopla: number;\
        \n  grarh: bool;\
        \n  f: hoo ha;\
        \n  fa: a, hey, hoo ha;\
        \n  fv: hoo ha;\
        \n  ft: arglist;\
        \n  fj: 1 2 3 hoo ha;\
        \n  fjt: list;\
        \n  fkt: list;\
        \n  hoopla: 5;\
        \n  boopla: number;\
        \n  koopla: number;\
        \n  new: string;\
        \n}\
        \ndiv {\
        \n  a: list;\
        \n  b: bool;\
        \n  c: number;\
        \n}\
        \n"
    );
}
