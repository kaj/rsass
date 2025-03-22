//! Tests auto-converted from "sass-spec/spec/libsass/bool.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("bool")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \ndiv {\
             \n  a: (false and \"hey\");\
             \n  b: (\"hey\" and \"ho\");\
             \n  b: (\"hey\" or \"ho\");\
             \n  a: false and \"hey\";\
             \n  b: \"hey\" and \"ho\";\
             \n  b: string.unquote(\"hey\") or \"ho\";\
             \n}"),
        "div {\
         \n  a: false;\
         \n  b: \"ho\";\
         \n  b: \"hey\";\
         \n  a: false;\
         \n  b: \"ho\";\
         \n  b: hey;\
         \n}\n"
    );
}
