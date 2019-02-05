//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/blead-global/functional"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/libsass/variable-scoping/blead-global/functional/each"
#[test]
#[ignore] // failing
fn each() {
    assert_eq!(
        rsass(
            "$root_default: initial;\r\n$root_implicit: initial;\r\n$root_explicit: initial !global;\r\n\r\n@function fn() {\r\n  @each $outer in 1 {\r\n    $root_implicit: outer;\r\n    $root_explicit: outer !global;\r\n    $root_default: outer !default;\r\n    $local_implicit: outer;\r\n    $local_explicit: outer !global;\r\n    $local_default: outer !default;\r\n    @each $inner in 2 {\r\n      $root_implicit: inner;\r\n      $root_explicit: inner !global;\r\n      $root_default: inner !default;\r\n      $local_implicit: inner;\r\n      $local_explicit: inner !global;\r\n      $local_default: inner !default;\r\n    }\r\n  }\r\n  $check_implicit: $root_implicit !global;\r\n  $check_explicit: $root_explicit !global;\r\n  $check_default: $root_default !global;\r\n  @return null;\r\n}\r\n\r\nresult {\r\n  fn: fn();\r\n  @if variable-exists(outer) {\r\n    outer: $outer;\r\n  }\r\n  @if variable-exists(inner) {\r\n    inner: $inner;\r\n  }\r\n  @if variable-exists(check_implicit) {\r\n    check_implicit: $check_implicit;\r\n  }\r\n  @if variable-exists(check_explicit) {\r\n    check_explicit: $check_explicit;\r\n  }\r\n  @if variable-exists(check_default) {\r\n    check_default: $check_default;\r\n  }\r\n  root_default: $root_default;\r\n  root_implicit: $root_implicit;\r\n  root_explicit: $root_explicit;\r\n  @if variable-exists(local_default) {\r\n    local_default: $local_default;\r\n  }\r\n  @if variable-exists(local_implicit) {\r\n    local_implicit: $local_implicit;\r\n  }\r\n  @if variable-exists(local_explicit) {\r\n    local_explicit: $local_explicit;\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "result {\n  check_implicit: initial;\n  check_explicit: inner;\n  check_default: initial;\n  root_default: initial;\n  root_implicit: initial;\n  root_explicit: inner;\n  local_explicit: inner;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/blead-global/functional/else"
#[test]
#[ignore] // failing
fn test_else() {
    assert_eq!(
        rsass(
            "$root_default: initial;\r\n$root_implicit: initial;\r\n$root_explicit: initial !global;\r\n\r\n@function fn() {\r\n  @if false {\r\n    // nothing\r\n  }\r\n  @else {\r\n    $root_implicit: outer;\r\n    $root_explicit: outer !global;\r\n    $root_default: outer !default;\r\n    $local_implicit: outer;\r\n    $local_explicit: outer !global;\r\n    $local_default: outer !default;\r\n    @if false {\r\n      // nothing\r\n    }\r\n    @else {\r\n      $root_implicit: inner;\r\n      $root_explicit: inner !global;\r\n      $root_default: inner !default;\r\n      $local_implicit: inner;\r\n      $local_explicit: inner !global;\r\n      $local_default: inner !default;\r\n    }\r\n  }\r\n  $check_implicit: $root_implicit !global;\r\n  $check_explicit: $root_explicit !global;\r\n  $check_default: $root_default !global;\r\n  @return null;\r\n}\r\n\r\nresult {\r\n  fn: fn();\r\n  @if variable-exists(outer) {\r\n    outer: $outer;\r\n  }\r\n  @if variable-exists(inner) {\r\n    inner: $inner;\r\n  }\r\n  @if variable-exists(check_implicit) {\r\n    check_implicit: $check_implicit;\r\n  }\r\n  @if variable-exists(check_explicit) {\r\n    check_explicit: $check_explicit;\r\n  }\r\n  @if variable-exists(check_default) {\r\n    check_default: $check_default;\r\n  }\r\n  root_default: $root_default;\r\n  root_implicit: $root_implicit;\r\n  root_explicit: $root_explicit;\r\n  @if variable-exists(local_default) {\r\n    local_default: $local_default;\r\n  }\r\n  @if variable-exists(local_implicit) {\r\n    local_implicit: $local_implicit;\r\n  }\r\n  @if variable-exists(local_explicit) {\r\n    local_explicit: $local_explicit;\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "result {\n  check_implicit: initial;\n  check_explicit: inner;\n  check_default: initial;\n  root_default: initial;\n  root_implicit: initial;\n  root_explicit: inner;\n  local_explicit: inner;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/blead-global/functional/elseif"
#[test]
#[ignore] // failing
fn elseif() {
    assert_eq!(
        rsass(
            "$root_default: initial;\r\n$root_implicit: initial;\r\n$root_explicit: initial !global;\r\n\r\n@function fn() {\r\n  @if false {\r\n    // nothing\r\n  }\r\n  @else if true {\r\n    $root_implicit: outer;\r\n    $root_explicit: outer !global;\r\n    $root_default: outer !default;\r\n    $local_implicit: outer;\r\n    $local_explicit: outer !global;\r\n    $local_default: outer !default;\r\n    @if false {\r\n      // nothing\r\n    }\r\n    @else if true {\r\n      $root_implicit: inner;\r\n      $root_explicit: inner !global;\r\n      $root_default: inner !default;\r\n      $local_implicit: inner;\r\n      $local_explicit: inner !global;\r\n      $local_default: inner !default;\r\n    }\r\n  }\r\n  $check_implicit: $root_implicit !global;\r\n  $check_explicit: $root_explicit !global;\r\n  $check_default: $root_default !global;\r\n  @return null;\r\n}\r\n\r\nresult {\r\n  fn: fn();\r\n  @if variable-exists(outer) {\r\n    outer: $outer;\r\n  }\r\n  @if variable-exists(inner) {\r\n    inner: $inner;\r\n  }\r\n  @if variable-exists(check_implicit) {\r\n    check_implicit: $check_implicit;\r\n  }\r\n  @if variable-exists(check_explicit) {\r\n    check_explicit: $check_explicit;\r\n  }\r\n  @if variable-exists(check_default) {\r\n    check_default: $check_default;\r\n  }\r\n  root_default: $root_default;\r\n  root_implicit: $root_implicit;\r\n  root_explicit: $root_explicit;\r\n  @if variable-exists(local_default) {\r\n    local_default: $local_default;\r\n  }\r\n  @if variable-exists(local_implicit) {\r\n    local_implicit: $local_implicit;\r\n  }\r\n  @if variable-exists(local_explicit) {\r\n    local_explicit: $local_explicit;\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "result {\n  check_implicit: initial;\n  check_explicit: inner;\n  check_default: initial;\n  root_default: initial;\n  root_implicit: initial;\n  root_explicit: inner;\n  local_explicit: inner;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/blead-global/functional/for"
#[test]
#[ignore] // failing
fn test_for() {
    assert_eq!(
        rsass(
            "$root_default: initial;\r\n$root_implicit: initial;\r\n$root_explicit: initial !global;\r\n\r\n@function fn() {\r\n  @for $outer from 1 to 2 {\r\n    $root_implicit: outer;\r\n    $root_explicit: outer !global;\r\n    $root_default: outer !default;\r\n    $local_implicit: outer;\r\n    $local_explicit: outer !global;\r\n    $local_default: outer !default;\r\n    @for $inner from 3 to 4 {\r\n      $root_implicit: inner;\r\n      $root_explicit: inner !global;\r\n      $root_default: inner !default;\r\n      $local_implicit: inner;\r\n      $local_explicit: inner !global;\r\n      $local_default: inner !default;\r\n    }\r\n  }\r\n  $check_implicit: $root_implicit !global;\r\n  $check_explicit: $root_explicit !global;\r\n  $check_default: $root_default !global;\r\n  @return null;\r\n}\r\n\r\nresult {\r\n  fn: fn();\r\n  @if variable-exists(outer) {\r\n    outer: $outer;\r\n  }\r\n  @if variable-exists(inner) {\r\n    inner: $inner;\r\n  }\r\n  @if variable-exists(check_implicit) {\r\n    check_implicit: $check_implicit;\r\n  }\r\n  @if variable-exists(check_explicit) {\r\n    check_explicit: $check_explicit;\r\n  }\r\n  @if variable-exists(check_default) {\r\n    check_default: $check_default;\r\n  }\r\n  root_default: $root_default;\r\n  root_implicit: $root_implicit;\r\n  root_explicit: $root_explicit;\r\n  @if variable-exists(local_default) {\r\n    local_default: $local_default;\r\n  }\r\n  @if variable-exists(local_implicit) {\r\n    local_implicit: $local_implicit;\r\n  }\r\n  @if variable-exists(local_explicit) {\r\n    local_explicit: $local_explicit;\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "result {\n  check_implicit: initial;\n  check_explicit: inner;\n  check_default: initial;\n  root_default: initial;\n  root_implicit: initial;\n  root_explicit: inner;\n  local_explicit: inner;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/blead-global/functional/if"
#[test]
#[ignore] // failing
fn test_if() {
    assert_eq!(
        rsass(
            "$root_default: initial;\r\n$root_implicit: initial;\r\n$root_explicit: initial !global;\r\n\r\n@function fn() {\r\n  @if true {\r\n    $root_implicit: outer;\r\n    $root_explicit: outer !global;\r\n    $root_default: outer !default;\r\n    $local_implicit: outer;\r\n    $local_explicit: outer !global;\r\n    $local_default: outer !default;\r\n    @if true {\r\n      $root_implicit: inner;\r\n      $root_explicit: inner !global;\r\n      $root_default: inner !default;\r\n      $local_implicit: inner;\r\n      $local_explicit: inner !global;\r\n      $local_default: inner !default;\r\n    }\r\n  }\r\n  $check_implicit: $root_implicit !global;\r\n  $check_explicit: $root_explicit !global;\r\n  $check_default: $root_default !global;\r\n  @return null;\r\n}\r\n\r\nresult {\r\n  fn: fn();\r\n  @if variable-exists(outer) {\r\n    outer: $outer;\r\n  }\r\n  @if variable-exists(inner) {\r\n    inner: $inner;\r\n  }\r\n  @if variable-exists(check_implicit) {\r\n    check_implicit: $check_implicit;\r\n  }\r\n  @if variable-exists(check_explicit) {\r\n    check_explicit: $check_explicit;\r\n  }\r\n  @if variable-exists(check_default) {\r\n    check_default: $check_default;\r\n  }\r\n  root_default: $root_default;\r\n  root_implicit: $root_implicit;\r\n  root_explicit: $root_explicit;\r\n  @if variable-exists(local_default) {\r\n    local_default: $local_default;\r\n  }\r\n  @if variable-exists(local_implicit) {\r\n    local_implicit: $local_implicit;\r\n  }\r\n  @if variable-exists(local_explicit) {\r\n    local_explicit: $local_explicit;\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "result {\n  check_implicit: initial;\n  check_explicit: inner;\n  check_default: initial;\n  root_default: initial;\n  root_implicit: initial;\n  root_explicit: inner;\n  local_explicit: inner;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/variable-scoping/blead-global/functional/while"
#[test]
fn test_while() {
    assert_eq!(
        rsass(
            "$continue_inner: true;\r\n$continue_outer: true;\r\n$root_default: initial;\r\n$root_implicit: initial;\r\n$root_explicit: initial !global;\r\n\r\n@function fn() {\r\n  @while $continue_outer {\r\n    $root_implicit: outer;\r\n    $root_explicit: outer !global;\r\n    $root_default: outer !default;\r\n    $local_implicit: outer;\r\n    $local_explicit: outer !global;\r\n    $local_default: outer !default;\r\n    @while $continue_inner {\r\n      $root_implicit: inner;\r\n      $root_explicit: inner !global;\r\n      $root_default: inner !default;\r\n      $local_implicit: inner;\r\n      $local_explicit: inner !global;\r\n      $local_default: inner !default;\r\n      $continue_inner: false;\r\n    }\r\n    $continue_outer: false;\r\n  }\r\n  $check_implicit: $root_implicit !global;\r\n  $check_explicit: $root_explicit !global;\r\n  $check_default: $root_default !global;\r\n  @return null;\r\n}\r\n\r\nresult {\r\n  fn: fn();\r\n  @if variable-exists(continue_outer) {\r\n    continue_outer: $continue_outer;\r\n  }\r\n  @if variable-exists(continue_inner) {\r\n    continue_inner: $continue_inner;\r\n  }\r\n  @if variable-exists(check_implicit) {\r\n    check_implicit: $check_implicit;\r\n  }\r\n  @if variable-exists(check_explicit) {\r\n    check_explicit: $check_explicit;\r\n  }\r\n  @if variable-exists(check_default) {\r\n    check_default: $check_default;\r\n  }\r\n  root_default: $root_default;\r\n  root_implicit: $root_implicit;\r\n  root_explicit: $root_explicit;\r\n  @if variable-exists(local_default) {\r\n    local_default: $local_default;\r\n  }\r\n  @if variable-exists(local_implicit) {\r\n    local_implicit: $local_implicit;\r\n  }\r\n  @if variable-exists(local_explicit) {\r\n    local_explicit: $local_explicit;\r\n  }\r\n}\r\n"
        )
        .unwrap(),
        "result {\n  continue_outer: true;\n  continue_inner: true;\n  check_implicit: initial;\n  check_explicit: inner;\n  check_default: initial;\n  root_default: initial;\n  root_implicit: initial;\n  root_explicit: inner;\n  local_explicit: inner;\n}\n"
    );
}
