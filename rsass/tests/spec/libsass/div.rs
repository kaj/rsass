//! Tests auto-converted from "sass-spec/spec/libsass/div.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("div")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n$x: 3/4;\
             \n$xs: hey 3/4 ho;\n\
             \ndiv {\
             \n  /* $x: 3/4 */\
             \n  a: $x;\
             \n  b: hey $x ho;\
             \n  /* $xs: hey 3/4 ho */\
             \n  c: $xs;\
             \n  d: list.nth($xs, 2);\
             \n  e: list.nth($xs, 2) == 0.75;\
             \n  f: meta.type-of(list.nth($xs, 2));\
             \n}"),
        "div {\
         \n  /* $x: 3/4 */\
         \n  a: 0.75;\
         \n  b: hey 0.75 ho;\
         \n  /* $xs: hey 3/4 ho */\
         \n  c: hey 3/4 ho;\
         \n  d: 0.75;\
         \n  e: true;\
         \n  f: number;\
         \n}\n"
    );
}
