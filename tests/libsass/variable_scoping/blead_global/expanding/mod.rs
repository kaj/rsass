//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/blead-global/expanding"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// Ignoring "at-root", not expected to work yet.

// Ignoring "each", not expected to work yet.

// Ignoring "else", not expected to work yet.

// Ignoring "elseif", not expected to work yet.

// Ignoring "for", not expected to work yet.

/// From "sass-spec/spec/libsass/variable-scoping/blead-global/expanding/function"
#[test]
fn function() {
    assert_eq!(
        rsass(
            "$continue_inner: true;\r\n$continue_outer: true;\r\n$root_default: initial;\r\n$root_implicit: initial;\r\n$root_explicit: initial !global;\r\n\r\n@function fn() {\r\n  $root_implicit: outer;\r\n  $root_explicit: outer !global;\r\n  $root_default: outer !default;\r\n  $local_implicit: outer;\r\n  $local_explicit: outer !global;\r\n  $local_default: outer !default;\r\n  @while $continue_inner {\r\n    $root_implicit: inner;\r\n    $root_explicit: inner !global;\r\n    $root_default: inner !default;\r\n    $local_implicit: inner;\r\n    $local_explicit: inner !global;\r\n    $local_default: inner !default;\r\n    $continue_inner: false;\r\n  }\r\n  $continue_outer: false;\r\n  @return null;\r\n}\r\n\r\nresult {\r\n  fn: fn();\r\n  @if variable-exists(continue_outer) {\r\n    continue_outer: $continue_outer;\r\n  }\r\n  @if variable-exists(continue_inner) {\r\n    continue_inner: $continue_inner;\r\n  }\r\n  root_default: $root_default;\r\n  root_implicit: $root_implicit;\r\n  root_explicit: $root_explicit;\r\n  @if variable-exists(local_default) {\r\n    local_default: $local_default;\r\n  }\r\n  @if variable-exists(local_implicit) {\r\n    local_implicit: $local_implicit;\r\n  }\r\n  @if variable-exists(local_explicit) {\r\n    local_explicit: $local_explicit;\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "result {\n  continue_outer: true;\n  continue_inner: true;\n  root_default: initial;\n  root_implicit: initial;\n  root_explicit: inner;\n  local_explicit: inner;\n}\n"
    );
}

// Ignoring "if", not expected to work yet.

/// From "sass-spec/spec/libsass/variable-scoping/blead-global/expanding/mixin"
#[test]
fn mixin() {
    assert_eq!(
        rsass(
            "$root_default: initial;\r\n$root_implicit: initial;\r\n$root_explicit: initial !global;\r\n\r\n@mixin set_variable_inner() {\r\n  $root_implicit: inner;\r\n  $root_explicit: inner !global;\r\n  $root_default: inner !default;\r\n  $local_implicit: inner;\r\n  $local_explicit: inner !global;\r\n  $local_default: inner !default;\r\n}\r\n\r\n@mixin set_variable_outer() {\r\n  $root_implicit: outer;\r\n  $root_explicit: outer !global;\r\n  $root_default: outer !default;\r\n  $local_implicit: outer;\r\n  $local_explicit: outer !global;\r\n  $local_default: outer !default;\r\n  @include set_variable_inner();\r\n}\r\n\r\n@include set_variable_outer();\r\n\r\nresult {\r\n  root_default: $root_default;\r\n  root_implicit: $root_implicit;\r\n  root_explicit: $root_explicit;\r\n  @if variable-exists(local_default) {\r\n    local_default: $local_default;\r\n  }\r\n  @if variable-exists(local_implicit) {\r\n    local_implicit: $local_implicit;\r\n  }\r\n  @if variable-exists(local_explicit) {\r\n    local_explicit: $local_explicit;\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "result {\n  root_default: initial;\n  root_implicit: initial;\n  root_explicit: inner;\n  local_explicit: inner;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/blead-global/expanding/ruleset"
#[test]
fn ruleset() {
    assert_eq!(
        rsass(
            "$root_default: initial;\r\n$root_implicit: initial;\r\n$root_explicit: initial !global;\r\n\r\nruleset {\r\n  $root_implicit: outer;\r\n  $root_explicit: outer !global;\r\n  $root_default: outer !default;\r\n  $local_implicit: outer;\r\n  $local_explicit: outer !global;\r\n  $local_default: outer !default;\r\n  inner {\r\n    $root_implicit: inner;\r\n    $root_explicit: inner !global;\r\n    $root_default: inner !default;\r\n    $local_implicit: inner;\r\n    $local_explicit: inner !global;\r\n    $local_default: inner !default;\r\n  }\r\n}\r\n\r\nresult {\r\n  root_default: $root_default;\r\n  root_implicit: $root_implicit;\r\n  root_explicit: $root_explicit;\r\n  @if variable-exists(local_default) {\r\n    local_default: $local_default;\r\n  }\r\n  @if variable-exists(local_implicit) {\r\n    local_implicit: $local_implicit;\r\n  }\r\n  @if variable-exists(local_explicit) {\r\n    local_explicit: $local_explicit;\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "result {\n  root_default: initial;\n  root_implicit: initial;\n  root_explicit: inner;\n  local_explicit: inner;\n}\n"
    );
}

// Ignoring "while", not expected to work yet.
