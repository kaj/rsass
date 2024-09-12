//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/opacity/alpha.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("alpha")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \nfoo {\
             \n  c0: color.opacity(rgba(0,0,0,0.0));\
             \n  c1: color.opacity(rgba(0,0,0,0.1));\
             \n  c2: color.opacity(rgba(0,0,0,0.2));\
             \n  c3: color.opacity(rgba(0,0,0,0.3));\
             \n  c4: color.opacity(rgba(0,0,0,0.4));\
             \n  c5: color.opacity(rgba(0,0,0,0.5));\
             \n  c6: color.opacity(rgba(0,0,0,0.6));\
             \n  c7: color.opacity(rgba(0,0,0,0.7));\
             \n  c8: color.opacity(rgba(0,0,0,0.8));\
             \n  c9: color.opacity(rgba(0,0,0,0.9));\
             \n  c10: color.opacity(rgba(0,0,0,1));\
             \n}\n"),
        "foo {\
         \n  c0: 0;\
         \n  c1: 0.1;\
         \n  c2: 0.2;\
         \n  c3: 0.3;\
         \n  c4: 0.4;\
         \n  c5: 0.5;\
         \n  c6: 0.6;\
         \n  c7: 0.7;\
         \n  c8: 0.8;\
         \n  c9: 0.9;\
         \n  c10: 1;\
         \n}\n"
    );
}
