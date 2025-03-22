//! Tests auto-converted from "sass-spec/spec/directives/mixin/custom_ident_include.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("custom_ident_include")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin __a() {b: c}\
             \nd {@include --a}\n"),
        "d {\
         \n  b: c;\
         \n}\n"
    );
}
