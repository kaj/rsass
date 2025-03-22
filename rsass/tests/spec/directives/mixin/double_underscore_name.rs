//! Tests auto-converted from "sass-spec/spec/directives/mixin/double_underscore_name.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("double_underscore_name")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin __a() {b: c}\
             \nd {@include __a}\n"),
        "d {\
         \n  b: c;\
         \n}\n"
    );
}
