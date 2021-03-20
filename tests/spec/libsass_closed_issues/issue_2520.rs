//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2520.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "// ----\r\
            \n// Sass (v3.4.21)\r\
            \n// Compass (v1.0.3)\r\
            \n// ----\r\
            \n\r\
            \n@function remove-modifiers($selector) {\r\
            \n    // convert selector to a string\r\
            \n    $selector: inspect(nth($selector, 1));\r\
            \n  \r\
            \n    $modifier: \'\';\r\
            \n  \r\
            \n    // Find out where the first modifier starts\r\
            \n    $modifier-index: str-index($selector, \'\"--\');\r\
            \n  \r\
            \n    @if $modifier-index {\r\
            \n      // Strip the first part of the selector up until the first modifier\r\
            \n      $modifier: str-slice($selector, $modifier-index);\r\
            \n      // Find out where the modifier ends\r\
            \n      $modifier-end: str-index($modifier, \'\"]\');\r\
            \n      // Isolate the modifier\r\
            \n      $modifier: str-slice($modifier, 0, $modifier-end);\r\
            \n      // Remove the modifier from the selector\r\
            \n      $selector: str-replace($selector, $modifier, \'\');\r\
            \n      // Remove junk characters\r\
            \n      $selector: str-replace($selector, \'[class*=]\', \'\');\r\
            \n      // Recurse the function to eliminate any remainig modifiers\r\
            \n      $selector: remove-modifiers($selector);\r\
            \n    }\r\
            \n  \r\
            \n    @return $selector;\r\
            \n  }\r\
            \n  \r\
            \n  @function remove-duplicates($list, $recursive: false) {\r\
            \n    $result: ();\r\
            \n    \r\
            \n    @each $item in $list {\r\
            \n      @if not index($result, $item) {\r\
            \n        @if length($item) > 1 and $recursive {\r\
            \n          $result: append($result, remove-duplicates($item, $recursive));\r\
            \n        }\r\
            \n        @else {\r\
            \n          $result: append($result, $item);\r\
            \n        }\r\
            \n      }\r\
            \n    }\r\
            \n    \r\
            \n    @return $result;\r\
            \n  }\r\
            \n  \r\
            \n  @function str-replace($string, $search, $replace) { \r\
            \n    $index: str-index($string, $search);\r\
            \n  \r\
            \n    @if $index {\r\
            \n      @return str-slice($string, 1, $index - 1) + $replace + str-replace(\r\
            \n        str-slice($string, $index + str-length($search)), $search, $replace\r\
            \n      );\r\
            \n    }\r\
            \n  \r\
            \n    @return $string;\r\
            \n  }\r\
            \n  \r\
            \n  @function module-tree($selector) {\r\
            \n    $parent-module: $module;\r\
            \n  \r\
            \n    // Remove any modifers\r\
            \n    $selectors: remove-modifiers($selector);\r\
            \n  \r\
            \n    // Remove any junk characters\r\
            \n    $selectors: str-replace($selectors, \'.\', \'\');\r\
            \n    $selectors: str-replace($selectors, \'[class*=\"--\', \'\');\r\
            \n    $selectors: str-replace($selectors, \'[class*=\"\', \'\');\r\
            \n    $selectors: str-replace($selectors, \'--\"]\', \'\');\r\
            \n    $selectors: str-replace($selectors, \'\"]\', \'\');\r\
            \n  \r\
            \n    // Spoof our modules into a list\r\
            \n    $selectors: str-replace($selectors, \' \', \', \');\r\
            \n    $selectors: selector-parse($selectors);\r\
            \n  \r\
            \n    @return $selectors;\r\
            \n  }\r\
            \n  \r\
            \n  @function list-remove($list, $value, $recursive: false) { \r\
            \n      $result: ();\r\
            \n  \r\
            \n      @for $i from 1 through length($list) {\r\
            \n          @if type-of(nth($list, $i)) == list and $recursive {\r\
            \n              $result: append($result, list-remove(nth($list, $i), $value, $recursive), comma);\r\
            \n          } @else if nth($list, $i) != $value {\r\
            \n              $result: append($result, nth($list, $i), comma);\r\
            \n          }\r\
            \n      }\r\
            \n  \r\
            \n      @return $result;\r\
            \n   }\r\
            \n   \r\
            \n  @function this($options...) {\r\
            \n    $value: map-get($config, $options...);\r\
            \n    $debug: true;\r\
            \n  \r\
            \n    $this: &;\r\
            \n  \r\
            \n    @if length($this) > 0 {\r\
            \n      @if str-index(inspect(nth($this, 1)), \'%\') == 1 {\r\
            \n        $debug: false;\r\
            \n      }\r\
            \n    }\r\
            \n  \r\
            \n    @if $debug and not $value and $value != false {\r\
            \n        @warn \'#{$options} not found in #{$module} config\';\r\
            \n    }\r\
            \n  \r\
            \n    @return $value;\r\
            \n  }\r\
            \n  \r\
            \n  @function config($map-old, $map-new) {\r\
            \n      // Merge default and custom options\r\
            \n      $map-merged: map-merge($map-old, $map-new);\r\
            \n    \r\
            \n      // Store config in global variable\r\
            \n      $config: $map-merged !global;\r\
            \n  \r\
            \n      // Return merged map\r\
            \n      @return $map-merged;\r\
            \n  }\r\
            \n  \r\
            \n  @mixin module($module: $module) {\r\
            \n    $nested: &;\r\
            \n  \r\
            \n    @if not $nested {\r\
            \n      $module: $module !global;\r\
            \n    }\r\
            \n    \r\
            \n    $selectors: ();\r\
            \n  \r\
            \n    @each $item in $module {\r\
            \n      $selectors: join($selectors, \'.#{$module}\', comma);\r\
            \n      $selectors: join($selectors, \'[class*=\"#{$module}--\"]\', comma);\r\
            \n    }\r\
            \n      \r\
            \n    #{$selectors} {\r\
            \n      @content;\r\
            \n    }\r\
            \n  }\r\
            \n  \r\
            \n  @mixin component($components: null, $glue: \'__\') {\r\
            \n      $selectors: \'[class*=\"#{$module}#{$glue}\"]\';\r\
            \n      $this: &;\r\
            \n      $tree: module-tree($this);\r\
            \n      $namespace: nth($tree, length($tree));\r\
            \n  \r\
            \n      @if $components {\r\
            \n        $selectors: ();\r\
            \n  \r\
            \n        @each $component in $components {\r\
            \n          $selectors: join(\r\
            \n            $selectors, \r\
            \n            \'.#{$namespace}#{$glue}#{$component}, [class*=\"#{$namespace}#{$glue}#{$component}-\"]\', \r\
            \n            comma\r\
            \n          );\r\
            \n        }\r\
            \n      }\r\
            \n  \r\
            \n      $parents: ();\r\
            \n  \r\
            \n      @each $selector in & {\r\
            \n        // spoof the selector into a list\r\
            \n        $selector: str-replace(inspect($selector), \' \', \', \');\r\
            \n        $selector: selector-parse($selector);\r\
            \n  \r\
            \n        // if the last item isn\'t a modifier, remove it\r\
            \n        @if not str-index(inspect(nth($selector, length($selector))), \'[class*=\"--\') {\r\
            \n          $selector: list-remove($selector, nth($selector, length($selector)));\r\
            \n        }\r\
            \n  \r\
            \n        // convert the selector back into a string\r\
            \n        @if length($selector) == 1 {\r\
            \n          $selector: nth($selector, 1);\r\
            \n        }\r\
            \n        $selector: str-replace(inspect($selector), \', \', \' \');\r\
            \n  \r\
            \n        // Re-build the parent selector\r\
            \n        $parents: append($parents, $selector, comma);\r\
            \n      }\r\
            \n  \r\
            \n      // remove duplicate selectors\r\
            \n      $parents: remove-duplicates($parents);\r\
            \n  \r\
            \n      @if length($parents) == 1 {\r\
            \n        $parents: nth($parents, 1);\r\
            \n      }\r\
            \n  \r\
            \n      @if ($parents == \'()\') {\r\
            \n        @at-root #{$selectors} {\r\
            \n          @content;\r\
            \n        }\r\
            \n      } @else {\r\
            \n        @at-root #{$parents} {\r\
            \n          #{$selectors} {\r\
            \n              @content;\r\
            \n          }\r\
            \n        }\r\
            \n      }\r\
            \n  \r\
            \n  }\r\
            \n  \r\
            \n  @mixin modifier($modifier) {\r\
            \n    $selectors: &;\r\
            \n  \r\
            \n    @if str-index(inspect(&), \'.#{$module}\') {\r\
            \n      $selectors: ();\r\
            \n  \r\
            \n      @for $i from 1 through length(&) {\r\
            \n        @if $i % 2 == 0 {\r\
            \n          $selectors: append($selectors, nth(&, $i), comma);\r\
            \n        }\r\
            \n      }\r\
            \n    }\r\
            \n  \r\
            \n    @at-root #{$selectors} {\r\
            \n      $modifier-selectors: ();\r\
            \n      \r\
            \n      @each $item in $modifier {\r\
            \n        $modifier-selectors: join(\r\
            \n          $modifier-selectors, \'&[class*=\"--#{$modifier}\"]\', comma\r\
            \n        );\r\
            \n      }\r\
            \n  \r\
            \n      #{$modifier-selectors} {\r\
            \n        @content;\r\
            \n      }\r\
            \n    }\r\
            \n  }\r\
            \n  \r\
            \n  @mixin button($custom:()) {\r\
            \n    $buttons: config((\r\
            \n      \'group-spacing\': 1em\r\
            \n    ), $custom);\r\
            \n  \r\
            \n    @include module(\'button\') {\r\
            \n      @include component(\'group\') {\r\
            \n        content: \'fizzbuzz\';\r\
            \n        @include module {\r\
            \n          margin-left: this(\'group-spacing\');\r\
            \n          &:first-child {\r\
            \n            margin-left: 0;\r\
            \n          }\r\
            \n        }\r\
            \n      }\r\
            \n    }\r\
            \n  }\r\
            \n  \r\
            \n  @include button();\r\
            \n  \r\
            \n  @include module(\'modal\') {\r\
            \n    @include modifier(\'animate\') {\r\
            \n      @include modifier(\'zoom\') {\r\
            \n        content: \"fizzbuzz\"\r\
            \n      }\r\
            \n    }\r\
            \n  }"
        )
        .unwrap(),
        ".button__group, [class*=\"button__group-\"] {\
        \n  content: \'fizzbuzz\';\
        \n}\
        \n.button__group .button, .button__group [class*=\"button--\"], [class*=\"button__group-\"] .button, [class*=\"button__group-\"] [class*=\"button--\"] {\
        \n  margin-left: 1em;\
        \n}\
        \n.button__group .button:first-child, .button__group [class*=\"button--\"]:first-child, [class*=\"button__group-\"] .button:first-child, [class*=\"button__group-\"] [class*=\"button--\"]:first-child {\
        \n  margin-left: 0;\
        \n}\
        \n[class*=\"modal--\"][class*=\"--animate\"][class*=\"--zoom\"] {\
        \n  content: \"fizzbuzz\";\
        \n}\
        \n"
    );
}
