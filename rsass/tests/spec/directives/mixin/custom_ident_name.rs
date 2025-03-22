//! Tests auto-converted from "sass-spec/spec/directives/mixin/custom_ident_name.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("custom_ident_name")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin --a {b: c}\
             \nd {@include --a}\n"),
        "d {\
         \n  b: c;\
         \n}\n"
    );
}
