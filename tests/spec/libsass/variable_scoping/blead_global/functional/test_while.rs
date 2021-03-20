//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/blead-global/functional/while.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$continue_inner: true;\r\
            \n$continue_outer: true;\r\
            \n$root_default: initial;\r\
            \n$root_implicit: initial;\r\
            \n$root_explicit: initial !global;\r\
            \n\r\
            \n@function fn() {\r\
            \n  @while $continue_outer {\r\
            \n    $root_implicit: outer;\r\
            \n    $root_explicit: outer !global;\r\
            \n    $root_default: outer !default;\r\
            \n    $local_implicit: outer;\r\
            \n    $local_explicit: outer !global;\r\
            \n    $local_default: outer !default;\r\
            \n    @while $continue_inner {\r\
            \n      $root_implicit: inner;\r\
            \n      $root_explicit: inner !global;\r\
            \n      $root_default: inner !default;\r\
            \n      $local_implicit: inner;\r\
            \n      $local_explicit: inner !global;\r\
            \n      $local_default: inner !default;\r\
            \n      $continue_inner: false;\r\
            \n    }\r\
            \n    $continue_outer: false;\r\
            \n  }\r\
            \n  $check_implicit: $root_implicit !global;\r\
            \n  $check_explicit: $root_explicit !global;\r\
            \n  $check_default: $root_default !global;\r\
            \n  @return null;\r\
            \n}\r\
            \n\r\
            \nresult {\r\
            \n  fn: fn();\r\
            \n  @if variable-exists(continue_outer) {\r\
            \n    continue_outer: $continue_outer;\r\
            \n  }\r\
            \n  @if variable-exists(continue_inner) {\r\
            \n    continue_inner: $continue_inner;\r\
            \n  }\r\
            \n  @if variable-exists(check_implicit) {\r\
            \n    check_implicit: $check_implicit;\r\
            \n  }\r\
            \n  @if variable-exists(check_explicit) {\r\
            \n    check_explicit: $check_explicit;\r\
            \n  }\r\
            \n  @if variable-exists(check_default) {\r\
            \n    check_default: $check_default;\r\
            \n  }\r\
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
        \n  continue_outer: true;\
        \n  continue_inner: true;\
        \n  check_implicit: initial;\
        \n  check_explicit: inner;\
        \n  check_default: initial;\
        \n  root_default: initial;\
        \n  root_implicit: initial;\
        \n  root_explicit: inner;\
        \n  local_explicit: inner;\
        \n}\
        \n"
    );
}
