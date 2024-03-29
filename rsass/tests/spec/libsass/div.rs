//! Tests auto-converted from "sass-spec/spec/libsass/div.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("div")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$x: 3/4;\
             \n$xs: hey 3/4 ho;\n\
             \ndiv {\
             \n  /* $x: 3/4 */\
             \n  a: $x;\
             \n  b: hey $x ho;\
             \n  /* $xs: hey 3/4 ho */\
             \n  c: $xs;\
             \n  d: nth($xs, 2);\
             \n  e: nth($xs, 2) == 0.75;\
             \n  f: type-of(nth($xs, 2));\
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
