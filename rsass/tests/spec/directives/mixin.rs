//! Tests auto-converted from "sass-spec/spec/directives/mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin")
}

#[test]
fn custom_ident_include() {
    assert_eq!(
        runner().ok("@mixin __a() {b: c}\
             \nd {@include --a}\n"),
        "d {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn custom_ident_name() {
    assert_eq!(
        runner().ok("@mixin --a {b: c}\
             \nd {@include --a}\n"),
        "d {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn double_underscore_name() {
    assert_eq!(
        runner().ok("@mixin __a() {b: c}\
             \nd {@include __a}\n"),
        "d {\
         \n  b: c;\
         \n}\n"
    );
}
