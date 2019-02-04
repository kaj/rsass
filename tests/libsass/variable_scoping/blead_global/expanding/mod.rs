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
            "$continue_inner: true;\n$continue_outer: true;\n$root_default: initial;\n$root_implicit: initial;\n$root_explicit: initial !global;\n@function fn() {\n  $root_implicit: outer;\n  $root_explicit: outer !global;\n  $root_default: outer !default;\n  $local_implicit: outer;\n  $local_explicit: outer !global;\n  $local_default: outer !default;\n  @while $continue_inner {\n    $root_implicit: inner;\n    $root_explicit: inner !global;\n    $root_default: inner !default;\n    $local_implicit: inner;\n    $local_explicit: inner !global;\n    $local_default: inner !default;\n    $continue_inner: false;\n  }\n  $continue_outer: false;\n  @return null;\n}\nresult {\n  fn: fn();\n  @if variable-exists(continue_outer) {\n    continue_outer: $continue_outer;\n  }\n  @if variable-exists(continue_inner) {\n    continue_inner: $continue_inner;\n  }\n  root_default: $root_default;\n  root_implicit: $root_implicit;\n  root_explicit: $root_explicit;\n  @if variable-exists(local_default) {\n    local_default: $local_default;\n  }\n  @if variable-exists(local_implicit) {\n    local_implicit: $local_implicit;\n  }\n  @if variable-exists(local_explicit) {\n    local_explicit: $local_explicit;\n  }\n}\n"
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
            "$root_default: initial;\n$root_implicit: initial;\n$root_explicit: initial !global;\n@mixin set_variable_inner() {\n  $root_implicit: inner;\n  $root_explicit: inner !global;\n  $root_default: inner !default;\n  $local_implicit: inner;\n  $local_explicit: inner !global;\n  $local_default: inner !default;\n}\n@mixin set_variable_outer() {\n  $root_implicit: outer;\n  $root_explicit: outer !global;\n  $root_default: outer !default;\n  $local_implicit: outer;\n  $local_explicit: outer !global;\n  $local_default: outer !default;\n  @include set_variable_inner();\n}\n@include set_variable_outer();\nresult {\n  root_default: $root_default;\n  root_implicit: $root_implicit;\n  root_explicit: $root_explicit;\n  @if variable-exists(local_default) {\n    local_default: $local_default;\n  }\n  @if variable-exists(local_implicit) {\n    local_implicit: $local_implicit;\n  }\n  @if variable-exists(local_explicit) {\n    local_explicit: $local_explicit;\n  }\n}\n"
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
            "$root_default: initial;\n$root_implicit: initial;\n$root_explicit: initial !global;\nruleset {\n  $root_implicit: outer;\n  $root_explicit: outer !global;\n  $root_default: outer !default;\n  $local_implicit: outer;\n  $local_explicit: outer !global;\n  $local_default: outer !default;\n  inner {\n    $root_implicit: inner;\n    $root_explicit: inner !global;\n    $root_default: inner !default;\n    $local_implicit: inner;\n    $local_explicit: inner !global;\n    $local_default: inner !default;\n  }\n}\nresult {\n  root_default: $root_default;\n  root_implicit: $root_implicit;\n  root_explicit: $root_explicit;\n  @if variable-exists(local_default) {\n    local_default: $local_default;\n  }\n  @if variable-exists(local_implicit) {\n    local_implicit: $local_implicit;\n  }\n  @if variable-exists(local_explicit) {\n    local_explicit: $local_explicit;\n  }\n}\n"
        )
        .unwrap(),
        "result {\n  root_default: initial;\n  root_implicit: initial;\n  root_explicit: inner;\n  local_explicit: inner;\n}\n"
    );
}

// Ignoring "while", not expected to work yet.
