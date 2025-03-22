//! Tests auto-converted from "sass-spec/spec/libsass/arg-eval.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("arg-eval")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@function foo() {\
             \n  @return 1+2 3/4 5+6;\
             \n}\n\
             \n@mixin bar($x: 3/4) {\
             \n  bar-content: $x;\
             \n}\n\
             \ndiv {\
             \n  content: foobar(1+2 3/4 5+6, orange);\
             \n  content: list.append(1+2 2/3 5+6, orange);\
             \n  content: 1+2 2/3 5+6;\
             \n  content: meta.type-of(2/3);\
             \n  content: meta.type-of(orange);\
             \n  content: foo();\
             \n  @include bar();\
             \n}"),
        "div {\
         \n  content: foobar(3 3/4 11, orange);\
         \n  content: 3 2/3 11 orange;\
         \n  content: 3 2/3 11;\
         \n  content: number;\
         \n  content: color;\
         \n  content: 3 3/4 11;\
         \n  bar-content: 0.75;\
         \n}\n"
    );
}
