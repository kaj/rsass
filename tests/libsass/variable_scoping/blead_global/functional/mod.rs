//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/blead-global/functional"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "each", not expected to work yet.

// Ignoring "else", not expected to work yet.

// Ignoring "elseif", not expected to work yet.

// Ignoring "for", not expected to work yet.

// Ignoring "if", not expected to work yet.

/// From "sass-spec/spec/libsass/variable-scoping/blead-global/functional/while"
#[test]
fn test_while() {
    assert_eq!(
        rsass(
            "$continue_inner: true;\n$continue_outer: true;\n$root_default: initial;\n$root_implicit: initial;\n$root_explicit: initial !global;\n@function fn() {\n  @while $continue_outer {\n    $root_implicit: outer;\n    $root_explicit: outer !global;\n    $root_default: outer !default;\n    $local_implicit: outer;\n    $local_explicit: outer !global;\n    $local_default: outer !default;\n    @while $continue_inner {\n      $root_implicit: inner;\n      $root_explicit: inner !global;\n      $root_default: inner !default;\n      $local_implicit: inner;\n      $local_explicit: inner !global;\n      $local_default: inner !default;\n      $continue_inner: false;\n    }\n    $continue_outer: false;\n  }\n  $check_implicit: $root_implicit !global;\n  $check_explicit: $root_explicit !global;\n  $check_default: $root_default !global;\n  @return null;\n}\nresult {\n  fn: fn();\n  @if variable-exists(continue_outer) {\n    continue_outer: $continue_outer;\n  }\n  @if variable-exists(continue_inner) {\n    continue_inner: $continue_inner;\n  }\n  @if variable-exists(check_implicit) {\n    check_implicit: $check_implicit;\n  }\n  @if variable-exists(check_explicit) {\n    check_explicit: $check_explicit;\n  }\n  @if variable-exists(check_default) {\n    check_default: $check_default;\n  }\n  root_default: $root_default;\n  root_implicit: $root_implicit;\n  root_explicit: $root_explicit;\n  @if variable-exists(local_default) {\n    local_default: $local_default;\n  }\n  @if variable-exists(local_implicit) {\n    local_implicit: $local_implicit;\n  }\n  @if variable-exists(local_explicit) {\n    local_explicit: $local_explicit;\n  }\n}\n"
        )
        .unwrap(),
        "result {\n  continue_outer: true;\n  continue_inner: true;\n  check_implicit: initial;\n  check_explicit: inner;\n  check_default: initial;\n  root_default: initial;\n  root_implicit: initial;\n  root_explicit: inner;\n  local_explicit: inner;\n}\n"
    );
}
