//! Tests auto-converted from "sass-spec/spec/non_conformant/mixin"
#[allow(unused)]
use super::rsass;

mod content;

// From "sass-spec/spec/non_conformant/mixin/environment_locality.hrx"
#[test]
fn environment_locality() {
    assert_eq!(
        rsass(
            "// The \"$var\" variable should only be set locally, despite being in the same\
            \n// mixin each time.\
            \n@mixin with-local-variable($recurse) {\
            \n  $var: before;\
            \n\
            \n  @if ($recurse) {\
            \n    @include with-local-variable($recurse: false);\
            \n  }\
            \n\
            \n  var: $var;\
            \n  $var: after;\
            \n}\
            \n\
            \n.environment-locality {\
            \n  @include with-local-variable($recurse: true);\
            \n}\
            \n"
        )
        .unwrap(),
        ".environment-locality {\
        \n  var: before;\
        \n  var: before;\
        \n}\
        \n"
    );
}

mod error;
