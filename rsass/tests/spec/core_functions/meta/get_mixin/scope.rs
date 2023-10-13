//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_mixin/scope.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("scope")
}

#[test]
#[ignore] // unexepected error
fn captures_inner_scope() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@mixin add-two($v) {@error \"Should not be called\"}\
             \n.scope1 {\
             \n  @mixin add-two($v) {@error \"Should not be called\"}\
             \n  .scope2 {\
             \n    @mixin add-two($v) {@error \"Should not be called\"}\
             \n    .scope3 {\
             \n      @mixin add-two($v) {a: $v + 2}\n\
             \n      // Like a normal mixin call, get-mixin() will always use the\
             \n      // innermost definition of a mixin.\
             \n      @include meta.apply(get-mixin(add-two), 10);\
             \n    }\
             \n  }\
             \n}\n"
        ),
        ".scope1 .scope2 .scope3 {\
         \n  a: 12;\
         \n}\n"
    );
}
mod scope {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn mutated_global() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n$a: x;\n\
             \n@mixin a {a: $a}\n\
             \n$ref: meta.get-mixin(\"a\");\n\
             \n$a: y;\n\
             \na {@include meta.apply($ref);}\n"),
            "a {\
         \n  a: y;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn mutated_local() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\n\
             \na {\
             \n  $a: x;\n\
             \n  @mixin a {\
             \n    a: $a;\
             \n  }\n\
             \n  $ref: meta.get-mixin(\"a\");\n\
             \n  $a: y;\
             \n  @include meta.apply($ref);\
             \n}\n"),
            "a {\
         \n  a: y;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn stores_local_scope() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n$add-two-mixin: null;\n\
             \n.scope {\
             \n  @mixin add-two($v) {b: $v + 2}\n\
             \n  // This mixin reference will still refer to this nested `add-two` mixin\
             \n  // even when it goes out of scope.\
             \n  $add-two-mixin: get-mixin(add-two) !global;\
             \n}\n\
             \na {@include meta.apply($add-two-mixin, 10)}\n"
        ),
        "a {\
         \n  b: 12;\
         \n}\n"
    );
}
