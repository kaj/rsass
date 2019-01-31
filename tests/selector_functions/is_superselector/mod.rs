//! Tests auto-converted from "sass-spec/spec/selector-functions/is_superselector"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/selector-functions/is_superselector/any_is_not_superselector_of_different_prefix"
#[test]
fn any_is_not_superselector_of_different_prefix() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: refute_superselector(\':-moz-any(.foo, .bar)\', \':-s-any(.foo, .bar)\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/child_isnt_superselector_of_longer_child"
#[test]
fn child_isnt_superselector_of_longer_child() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: refute_superselector(\'.foo > .baz\', \'.foo > .bar > .baz\');\n  b: refute_superselector(\'.foo > .baz\', \'.foo > .bar .baz\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/complex_superselector"
#[test]
fn complex_superselector() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a : assert_strict_superselector(\'.bar\', \'.foo .bar\');\n  b : assert_strict_superselector(\'.bar\', \'.foo > .bar\');\n  c : assert_strict_superselector(\'.bar\', \'.foo + .bar\');\n  d : assert_strict_superselector(\'.bar\', \'.foo ~ .bar\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n  c: true;\n  d: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/compound_superselector"
#[test]
fn compound_superselector() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a : assert_strict_superselector(\'.foo\', \'.foo.bar\');\n  b : assert_strict_superselector(\'.bar\', \'.foo.bar\');\n  c : assert_strict_superselector(\'a\', \'a#b\');\n  d : assert_strict_superselector(\'#b\', \'a#b\');\n}\n"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n  c: true;\n  d: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/current_is_superselector_with_identical_innards"
#[test]
fn current_is_superselector_with_identical_innards() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: is-superselector(\':current(.foo)\', \':current(.foo)\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/current_is_superselector_with_subselector_innards"
#[test]
fn current_is_superselector_with_subselector_innards() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: not is-superselector(\':current(.foo)\', \':current(.foo.bar)\');\n  b: not is-superselector(\':current(.foo.bar)\', \':current(.foo)\')\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/descendant_is_superselector_of_child"
#[test]
fn descendant_is_superselector_of_child() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: assert_strict_superselector(\'.foo .bar\', \'.foo > .bar.baz\');\n  b: assert_strict_superselector(\'.foo .bar\', \'.foo.baz > .bar\');\n  c: assert_strict_superselector(\'.foo .baz\', \'.foo > .bar > .baz\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n  c: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/following_sibling_is_superselector_of_next_sibling"
#[test]
fn following_sibling_is_superselector_of_next_sibling() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: assert_strict_superselector(\'.foo ~ .bar\', \'.foo + .bar.baz\');\n  b: assert_strict_superselector(\'.foo ~ .bar\', \'.foo.baz + .bar\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/following_sibling_isnt_superselector_of_longer_following_sibling"
#[test]
fn following_sibling_isnt_superselector_of_longer_following_sibling() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: refute_superselector(\'.foo + .baz\', \'.foo + .bar + .baz\');\n  b: refute_superselector(\'.foo + .baz\', \'.foo + .bar .baz\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/has_is_superselector_of_subset_host"
#[test]
fn has_is_superselector_of_subset_host() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: assert_strict_superselector(\':has(.foo, .bar, .baz)\', \':has(.foo.bip, .baz.bang)\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/host_context_is_superselector_of_subset_host"
#[test]
fn host_context_is_superselector_of_subset_host() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a:  assert_strict_superselector(\':host-context(.foo, .bar, .baz)\', \':host-context(.foo.bip, .baz.bang)\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/host_is_superselector_of_subset_host"
#[test]
fn host_is_superselector_of_subset_host() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: assert_strict_superselector(\':host(.foo, .bar, .baz)\', \':host(.foo.bip, .baz.bang)\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/leading_combinator"
#[test]
fn leading_combinator() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: refute_superselector(\'+ .foo\', \'.foo\');\n  b: refute_superselector(\'+ .foo\', \'.bar + .foo\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/matches_can_be_subselector"
#[test]
fn matches_can_be_subselector() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\n\n@function check_matches($a, $b) {\n  $prefixes: matches -moz-any;\n  $result: true;\n  @each $name in $prefixes{\n    @if (not assert_strict_superselector(\":#{$name}(#{$a})\", #{$b}) and $result == true) {\n      $result: false;\n    }\n  }\n\n  @return $result;\n}\n\ntest {\n  a: check_matches(\'.foo\', \'.foo.bar\');\n  b: check_matches(\'.foo, .bar\', \'.foo.bar.baz\');\n  c: check_matches(\'.foo\', \'.foo.bar, .foo.baz\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n  c: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/matches_is_not_superselector_of_any"
#[test]
fn matches_is_not_superselector_of_any() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: refute_superselector(\':matches(.foo, .bar)\', \':-moz-any(.foo, .bar)\');\n  b: refute_superselector(\':-moz-any(.foo, .bar)\', \':matches(.foo, .bar)\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/matches_is_superselector_of_constituent_selectors"
#[test]
fn matches_is_superselector_of_constituent_selectors() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\n\n@function check_matches($a, $b) {\n  $prefixes: matches -moz-any;\n  $result: true;\n  @each $name in $prefixes{\n    @if (not assert_strict_superselector(\":#{$name}(#{$a})\", #{$b}) and $result == true) {\n      $result: false;\n    }\n  }\n\n  @return $result;\n}\n\ntest {\n  a: check_matches(\'.foo, .bar\', \'.foo.baz\');\n  b: check_matches(\'.foo, .bar\', \'.bar.baz\');\n  c: check_matches(\".foo .bar, .baz\", \'.x .foo .bar\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n  c: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/matches_is_superselector_of_subset_matches"
#[test]
fn matches_is_superselector_of_subset_matches() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: assert_strict_superselector(\':matches(.foo, .bar, .baz)\', \'#x:matches(.foo.bip, .baz.bang)\');\n  b: assert_strict_superselector(\':-moz-any(.foo, .bar, .baz)\', \'#x:-moz-any(.foo.bip, .baz.bang)\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/matching_combinator"
#[test]
fn matching_combinator() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: assert_strict_superselector(\'.foo + .bar\', \'.foo + .bar.baz\');\n  b: assert_strict_superselector(\'.foo + .bar\', \'.foo.baz + .bar\');\n  c: assert_strict_superselector(\'.foo > .bar\', \'.foo > .bar.baz\');\n  d: assert_strict_superselector(\'.foo > .bar\', \'.foo.baz > .bar\');\n  e: assert_strict_superselector(\'.foo ~ .bar\', \'.foo ~ .bar.baz\');\n  f: assert_strict_superselector(\'.foo ~ .bar\', \'.foo.baz ~ .bar\');\n}\n"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n  c: true;\n  d: true;\n  e: true;\n  f: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/not_is_not_superselector_of_non_unique_selectors"
#[test]
fn not_is_not_superselector_of_non_unique_selectors() {
    assert_eq!(
        rsass(
            "test {\n  a: not is-superselector(\':not(.foo)\', \'.bar\');\n  b: not is-superselector(\':not(:hover)\', \':visited\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/nth_match_is_not_superselector_of_nth_last_match"
#[test]
fn nth_match_is_not_superselector_of_nth_last_match() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: refute_superselector(\':nth-child(2n of .foo, .bar)\', \':nth-last-child(2n of .foo, .bar)\');\n  b: refute_superselector(\':nth-last-child(2n of .foo, .bar)\', \':nth-child(2n of .foo, .bar)\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/nth_match_is_not_superselector_of_nth_match_with_different_arg"
#[test]
fn nth_match_is_not_superselector_of_nth_match_with_different_arg() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: refute_superselector(\':nth-child(2n of .foo, .bar, .baz)\', \'#x:nth-child(2n + 1 of .foo.bip, .baz.bang)\');\n  b: refute_superselector(\':nth-last-child(2n of .foo, .bar, .baz)\', \'#x:nth-last-child(2n + 1 of .foo.bip, .baz.bang)\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/reflexivity"
#[test]
fn reflexivity() {
    assert_eq!(
        rsass(
            "test {\n   tag: is_superselector(\'h1\', \'h1\');\n   class: is_superselector(\'.foo\', \'.foo\');\n   descendant: is_superselector(\'#foo > .bar, baz\', \'#foo > .bar, baz\');\n }"
        )
        .unwrap(),
        "test {\n  tag: true;\n  class: true;\n  descendant: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/selector_list_subset"
#[test]
fn selector_list_subset() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: assert_strict_superselector(\'.foo, .bar\', \'.foo\');\n  b: assert_strict_superselector(\'.foo, .bar, .baz\', \'.foo, .baz\');\n  c: assert_strict_superselector(\'.foo, .baz, .qux\', \'.foo.bar, .baz.bang\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n  c: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/sibling_isnt_superselector_of_longer_sibling"
#[test]
fn sibling_isnt_superselector_of_longer_sibling() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\n// This actually is a superselector, but it\'s a very narrow edge case and\n// detecting it is very difficult and may be exponential in the worst case.\ntest {\n  a: refute_superselector(\'.foo ~ .baz\', \'.foo ~ .bar ~ .baz\');\n  b: refute_superselector(\'.foo ~ .baz\', \'.foo ~ .bar .baz\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/trailing_combinator"
#[test]
fn trailing_combinator() {
    assert_eq!(
        rsass(
            "@import \"../assert_helpers\";\n\ntest {\n  a: refute_superselector(\'.foo +\', \'.foo\');\n  b: refute_superselector(\'.foo +\', \'.foo + .bar\');\n}"
        )
        .unwrap(),
        "test {\n  a: true;\n  b: true;\n}\n"
    );
}

/// From "sass-spec/spec/selector-functions/is_superselector/universal_superselector"
#[test]
fn universal_superselector() {
    assert_eq!(
        rsass(
            "universal-selector {\r\n  test-01: is-superselector(\"*\", \"*\");\r\n  test-02: is-superselector(\"|*\", \"*\");\r\n  test-03: is-superselector(\"*|*\", \"*\");\r\n  test-04: is-superselector(\"*\", \"|*\");\r\n  test-05: is-superselector(\"|*\", \"|*\");\r\n  test-06: is-superselector(\"*|*\", \"|*\");\r\n  test-07: is-superselector(\"*\", \"*|*\");\r\n  test-08: is-superselector(\"|*\", \"*|*\");\r\n  test-09: is-superselector(\"*|*\", \"*|*\");\r\n}\r\n"
        )
        .unwrap(),
        "universal-selector {\n  test-01: true;\n  test-02: false;\n  test-03: false;\n  test-04: false;\n  test-05: true;\n  test-06: false;\n  test-07: false;\n  test-08: false;\n  test-09: true;\n}\n"
    );
}
