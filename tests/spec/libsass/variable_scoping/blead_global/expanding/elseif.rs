//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/blead-global/expanding/elseif.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$root_default: initial;\r\
            \n$root_implicit: initial;\r\
            \n$root_explicit: initial !global;\r\
            \n\r\
            \n@if false {\r\
            \n  // nothing\r\
            \n}\r\
            \n@else if true {\r\
            \n  $root_implicit: outer;\r\
            \n  $root_explicit: outer !global;\r\
            \n  $root_default: outer !default;\r\
            \n  $local_implicit: outer;\r\
            \n  $local_explicit: outer !global;\r\
            \n  $local_default: outer !default;\r\
            \n  @if false {\r\
            \n    // nothing\r\
            \n  }\r\
            \n  @else if true {\r\
            \n    $root_implicit: inner;\r\
            \n    $root_explicit: inner !global;\r\
            \n    $root_default: inner !default;\r\
            \n    $local_implicit: inner;\r\
            \n    $local_explicit: inner !global;\r\
            \n    $local_default: inner !default;\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \nresult {\r\
            \n  root_default: $root_default;\r\
            \n  root_implicit: $root_implicit;\r\
            \n  root_explicit: $root_explicit;\r\
            \n  @if variable-exists(local_default) {\r\
            \n    local_default: $local_default;\r\
            \n  }\r\
            \n  @if variable-exists(local_implicit) {\r\
            \n    local_implicit: $local_implicit;\r\
            \n  }\r\
            \n  @if variable-exists(local_explicit) {\r\
            \n    local_explicit: $local_explicit;\r\
            \n  }\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "result {\
        \n  root_default: initial;\
        \n  root_implicit: inner;\
        \n  root_explicit: inner;\
        \n  local_explicit: inner;\
        \n}\
        \n"
    );
}
