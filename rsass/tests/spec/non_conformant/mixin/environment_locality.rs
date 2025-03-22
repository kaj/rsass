//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin/environment_locality.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("environment_locality")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "// The \"$var\" variable should only be set locally, despite being in the same\
             \n// mixin each time.\
             \n@mixin with-local-variable($recurse) {\
             \n  $var: before;\n\
             \n  @if ($recurse) {\
             \n    @include with-local-variable($recurse: false);\
             \n  }\n\
             \n  var: $var;\
             \n  $var: after;\
             \n}\n\
             \n.environment-locality {\
             \n  @include with-local-variable($recurse: true);\
             \n}\n"
        ),
        ".environment-locality {\
         \n  var: before;\
         \n  var: before;\
         \n}\n"
    );
}
