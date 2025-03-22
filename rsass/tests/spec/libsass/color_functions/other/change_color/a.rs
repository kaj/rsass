//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/other/change-color/a.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("a")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \nfoo {\
             \n  // c-1: change-color(black,$alpha:-1);\
             \n  c0: color.change(black,$alpha:0.0);\
             \n  c1: color.change(black,$alpha:0.1);\
             \n  c2: color.change(black,$alpha:0.2);\
             \n  c3: color.change(black,$alpha:0.3);\
             \n  c4: color.change(black,$alpha:0.4);\
             \n  c5: color.change(black,$alpha:0.5);\
             \n  c6: color.change(black,$alpha:0.6);\
             \n  c7: color.change(black,$alpha:0.7);\
             \n  c8: color.change(black,$alpha:0.8);\
             \n  c9: color.change(black,$alpha:0.9);\
             \n  c10: color.change(black,$alpha:1);\
             \n  // c11: change-color(black,$alpha:1.1);\
             \n  // c12: change-color(black,$alpha:2);\
             \n}\n"),
        "foo {\
         \n  c0: rgba(0, 0, 0, 0);\
         \n  c1: rgba(0, 0, 0, 0.1);\
         \n  c2: rgba(0, 0, 0, 0.2);\
         \n  c3: rgba(0, 0, 0, 0.3);\
         \n  c4: rgba(0, 0, 0, 0.4);\
         \n  c5: rgba(0, 0, 0, 0.5);\
         \n  c6: rgba(0, 0, 0, 0.6);\
         \n  c7: rgba(0, 0, 0, 0.7);\
         \n  c8: rgba(0, 0, 0, 0.8);\
         \n  c9: rgba(0, 0, 0, 0.9);\
         \n  c10: black;\
         \n}\n"
    );
}
