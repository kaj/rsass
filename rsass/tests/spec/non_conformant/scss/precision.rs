//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/precision.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("precision")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  a: (20/3);\
             \n  b: (5/2);\
             \n  c: (9/3);\
             \n  d: (20/-3);\
             \n  e: (-5/2);\
             \n  f: -(9/3);\
             \n}"),
        "div {\
         \n  a: 6.6666666667;\
         \n  b: 2.5;\
         \n  c: 3;\
         \n  d: -6.6666666667;\
         \n  e: -2.5;\
         \n  f: -3;\
         \n}\n"
    );
}
