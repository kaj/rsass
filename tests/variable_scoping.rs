extern crate rsass;
use rsass::{compile_scss, OutputStyle};

/// spec/libsass/variable-scoping/blead-global/expanding/ruleset
#[test]
fn ruleset() {
    check(
        b"$root_default: initial;\n\
            $root_implicit: initial;\n\
            $root_explicit: initial !global;\n\n\
            ruleset {\n  $root_implicit: outer;\n  \
            $root_explicit: outer !global;\n  \
            $root_default: outer !default;\n  $local_implicit: outer;\n  \
            $local_explicit: outer !global;\n  \
            $local_default: outer !default;\n  \
            inner {\n    $root_implicit: inner;\n    \
            $root_explicit: inner !global;\n    \
            $root_default: inner !default;\n    $local_implicit: inner;\n    \
            $local_explicit: inner !global;\n    \
            $local_default: inner !default;\n  }\n}\n\n\
            result {\n  root_default: $root_default;\n  \
            root_implicit: $root_implicit;\n  \
            root_explicit: $root_explicit;\n  \
            @if variable-exists(local_default) {\n    \
            local_default: $local_default;\n  }\n  \
            @if variable-exists(local_implicit) {\n    \
            local_implicit: $local_implicit;\n  }\n  \
            @if variable-exists(local_explicit) {\n    \
            local_explicit: $local_explicit;\n  }\n}\n",
        "result {\n  root_default: initial;\n  root_implicit: initial;\n  \
         root_explicit: inner;\n  local_explicit: inner;\n}\n",
    )
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(
        compile_scss(input, OutputStyle::Normal)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        expected
    );
}
