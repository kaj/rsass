//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/scss-tests/001_test_one_line_comments.hrx"
#[test]
fn t001_test_one_line_comments() {
    assert_eq!(
        rsass(
            ".foo {// bar: baz;}\
            \n  baz: bang; //}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  baz: bang;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/002_test_one_line_comments.hrx"
#[test]
fn t002_test_one_line_comments() {
    assert_eq!(
        rsass(
            ".foo bar[val=\"//\"] {\
            \n  baz: bang; //}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo bar[val=\"//\"] {\
        \n  baz: bang;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/003_test_variables.hrx"
#[test]
fn t003_test_variables() {
    assert_eq!(
        rsass(
            "$var: foo;\
            \n\
            \nblat {a: $var}\
            \n"
        )
        .unwrap(),
        "blat {\
        \n  a: foo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/004_test_variables.hrx"
#[test]
fn t004_test_variables() {
    assert_eq!(
        rsass(
            "foo {\
            \n  $var: 2;\
            \n  $another-var: 4;\
            \n  a: $var;\
            \n  b: $var + $another-var;}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 2;\
        \n  b: 6;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/005_test_unicode_variables"
#[test]
fn t005_test_unicode_variables() {
    assert_eq!(
        rsass(
            "$vär: foo;\
            \n\
            \nblat {a: $vär}\
            \n"
        )
        .unwrap(),
        "blat {\
        \n  a: foo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/006_test_guard_assign.hrx"
#[test]
fn t006_test_guard_assign() {
    assert_eq!(
        rsass(
            "$var: 1;\
            \n$var: 2 !default;\
            \n\
            \nfoo {a: $var}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/007_test_guard_assign.hrx"
#[test]
fn t007_test_guard_assign() {
    assert_eq!(
        rsass(
            "$var: 2 !default;\
            \n\
            \nfoo {a: $var}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/008_test_sass_script.hrx"
#[test]
fn t008_test_sass_script() {
    assert_eq!(
        rsass(
            "foo {\
            \n  a: 1 + 2;\
            \n  b: 1 - 2;\
            \n  c: foo + bar;\
            \n  d: floor(12.3px); }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 3;\
        \n  b: -1;\
        \n  c: foobar;\
        \n  d: 12px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/009_test_for_directive.hrx"
#[test]
fn t009_test_for_directive() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  @for $var from 1 to 5 {a: $var;}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  a: 2;\
        \n  a: 3;\
        \n  a: 4;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/010_test_for_directive.hrx"
#[test]
fn t010_test_for_directive() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  @for $var from 1 through 5 {a: $var;}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  a: 2;\
        \n  a: 3;\
        \n  a: 4;\
        \n  a: 5;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/011_test_if_directive.hrx"
#[test]
fn t011_test_if_directive() {
    assert_eq!(
        rsass(
            "@if \"foo\" == \"foo\" {foo {a: b}}\
            \n@if \"foo\" != \"foo\" {bar {a: b}}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/012_test_if_directive.hrx"
#[test]
fn t012_test_if_directive() {
    assert_eq!(
        rsass(
            "@if \"foo\" != \"foo\" {foo {a: b}}\
            \n@else if \"foo\" == \"foo\" {bar {a: b}}\
            \n@else if true {baz {a: b}}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/013_test_if_directive.hrx"
#[test]
fn t013_test_if_directive() {
    assert_eq!(
        rsass(
            "@if \"foo\" != \"foo\" {foo {a: b}}\
            \n@else {bar {a: b}}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/014_test_comment_after_if_directive.hrx"
#[test]
fn t014_test_comment_after_if_directive() {
    assert_eq!(
        rsass(
            "foo {\
            \n  @if true {a: b}\
            \n  /* This is a comment */\
            \n  c: d }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: b;\
        \n  /* This is a comment */\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/015_test_comment_after_if_directive.hrx"
#[test]
fn t015_test_comment_after_if_directive() {
    assert_eq!(
        rsass(
            "foo {\
            \n  @if true {a: b}\
            \n  @else {x: y}\
            \n  /* This is a comment */\
            \n  c: d }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: b;\
        \n  /* This is a comment */\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/017_test_each_directive.hrx"
#[test]
fn t017_test_each_directive() {
    assert_eq!(
        rsass(
            "a {\
            \n  @each $number in 1px 2px 3px 4px {\
            \n    b: $number;\
            \n  }\
            \n}\
            \nc {\
            \n  @each $str in foo, bar, baz, bang {\
            \n    d: $str;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1px;\
        \n  b: 2px;\
        \n  b: 3px;\
        \n  b: 4px;\
        \n}\
        \nc {\
        \n  d: foo;\
        \n  d: bar;\
        \n  d: baz;\
        \n  d: bang;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/019_test_css_import_directive.hrx"
#[test]
fn t019_test_css_import_directive() {
    assert_eq!(
        rsass("@import \"foo.css\";").unwrap(),
        "@import \"foo.css\";\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/020_test_css_import_directive.hrx"
#[test]
fn t020_test_css_import_directive() {
    assert_eq!(
        rsass("@import \'foo.css\';").unwrap(),
        "@import \'foo.css\';\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/021_test_css_import_directive.hrx"
#[test]
fn t021_test_css_import_directive() {
    assert_eq!(
        rsass("@import url(\"foo.css\");").unwrap(),
        "@import url(\"foo.css\");\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/022_test_css_import_directive.hrx"
#[test]
fn t022_test_css_import_directive() {
    assert_eq!(
        rsass("@import url(\"foo.css\");").unwrap(),
        "@import url(\"foo.css\");\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/023_test_css_import_directive.hrx"
#[test]
fn t023_test_css_import_directive() {
    assert_eq!(
        rsass("@import url(foo.css);").unwrap(),
        "@import url(foo.css);\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/024_test_media_import.hrx"
#[test]
fn t024_test_media_import() {
    assert_eq!(
        rsass("@import \"./fonts.sass\" all;").unwrap(),
        "@import \"./fonts.sass\" all;\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/025_test_dynamic_media_import.hrx"
#[test]
fn t025_test_dynamic_media_import() {
    assert_eq!(
        rsass(
            "$media: print;\
            \n$key: -webkit-min-device-pixel-ratio;\
            \n$value: 20;\
            \n@import \"foo\" #{$media} and ($key + \"-foo\": $value + 5);\
            \n"
        )
        .unwrap(),
        "@import \"foo\" print and (-webkit-min-device-pixel-ratio-foo: 25);\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/027_test_protocol_relative_import.hrx"
#[test]
fn t027_test_protocol_relative_import() {
    assert_eq!(
        rsass("@import \"//fonts.googleapis.com/css?family=Droid+Sans\";")
            .unwrap(),
        "@import \"//fonts.googleapis.com/css?family=Droid+Sans\";\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/028_test_import_with_interpolation.hrx"
#[test]
fn t028_test_import_with_interpolation() {
    assert_eq!(
        rsass(
            "$family: unquote(\"Droid+Sans\");\
            \n@import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\
            \n"
        )
        .unwrap(),
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/029_test_url_import.hrx"
#[test]
fn t029_test_url_import() {
    assert_eq!(
        rsass("@import url(fonts.sass);").unwrap(),
        "@import url(fonts.sass);\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/030_test_block_comment_in_script.hrx"
#[test]
fn t030_test_block_comment_in_script() {
    assert_eq!(
        rsass(
            "foo {a: 1 + /* flang */ bar}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 1bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/031_test_line_comment_in_script.hrx"
#[test]
fn t031_test_line_comment_in_script() {
    assert_eq!(
        rsass(
            "foo {a: 1 + // flang }\
            \n  blang }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 1blang;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/032_test_nested_rules.hrx"
#[test]
fn t032_test_nested_rules() {
    assert_eq!(
        rsass(
            "foo {bar {a: b}}\
            \n"
        )
        .unwrap(),
        "foo bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/033_test_nested_rules.hrx"
#[test]
fn t033_test_nested_rules() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar {a: b}\
            \n  baz {b: c}}\
            \n"
        )
        .unwrap(),
        "foo bar {\
        \n  a: b;\
        \n}\
        \nfoo baz {\
        \n  b: c;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/034_test_nested_rules.hrx"
#[test]
fn t034_test_nested_rules() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar {baz {a: b}}\
            \n  bang {bip {a: b}}}\
            \n"
        )
        .unwrap(),
        "foo bar baz {\
        \n  a: b;\
        \n}\
        \nfoo bang bip {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/035_test_nested_rules_with_declarations.hrx"
#[test]
fn t035_test_nested_rules_with_declarations() {
    assert_eq!(
        rsass(
            "foo {\
            \n  a: b;\
            \n  bar {c: d}}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: b;\
        \n}\
        \nfoo bar {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/036_test_nested_rules_with_declarations.hrx"
#[test]
fn t036_test_nested_rules_with_declarations() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar {c: d}\
            \n  a: b}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: b;\
        \n}\
        \nfoo bar {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/037_test_nested_rules_with_declarations.hrx"
#[test]
fn t037_test_nested_rules_with_declarations() {
    assert_eq!(
        rsass(
            "foo {\
            \n  ump: nump;\
            \n  grump: clump;\
            \n  bar {\
            \n    blat: bang;\
            \n    habit: rabbit;\
            \n    baz {a: b}\
            \n    bip {c: d}}\
            \n  bibble {\
            \n    bap {e: f}}}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  ump: nump;\
        \n  grump: clump;\
        \n}\
        \nfoo bar {\
        \n  blat: bang;\
        \n  habit: rabbit;\
        \n}\
        \nfoo bar baz {\
        \n  a: b;\
        \n}\
        \nfoo bar bip {\
        \n  c: d;\
        \n}\
        \nfoo bibble bap {\
        \n  e: f;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/038_test_nested_rules_with_fancy_selectors.hrx"
#[test]
fn t038_test_nested_rules_with_fancy_selectors() {
    assert_eq!(
        rsass(
            "foo {\
            \n  .bar {a: b}\
            \n  :baz {c: d}\
            \n  bang:bop {e: f}}\
            \n"
        )
        .unwrap(),
        "foo .bar {\
        \n  a: b;\
        \n}\
        \nfoo :baz {\
        \n  c: d;\
        \n}\
        \nfoo bang:bop {\
        \n  e: f;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/039_test_almost_ambiguous_nested_rules_and_declarations.hrx"
#[test]
fn t039_test_almost_ambiguous_nested_rules_and_declarations() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar:baz:bang:bop:biddle:woo:look:at:all:these:pseudoclasses {a: b};\
            \n  bar:baz bang bop biddle woo look at all these elems {a: b};\
            \n  bar:baz bang bop biddle woo look at all these elems; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz bang bop biddle woo look at all these elems;\
        \n}\
        \nfoo bar:baz:bang:bop:biddle:woo:look:at:all:these:pseudoclasses {\
        \n  a: b;\
        \n}\
        \nfoo bar:baz bang bop biddle woo look at all these elems {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/040_test_newlines_in_selectors.hrx"
#[test]
#[ignore] // wrong result
fn t040_test_newlines_in_selectors() {
    assert_eq!(
        rsass(
            "foo\
            \nbar {a: b}\
            \n"
        )
        .unwrap(),
        "foo\
        \nbar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/041_test_newlines_in_selectors.hrx"
#[test]
#[ignore] // wrong result
fn t041_test_newlines_in_selectors() {
    assert_eq!(
        rsass(
            "foo,\
            \nbar {\
            \n  baz,\
            \n  bang {a: b}}\
            \n"
        )
        .unwrap(),
        "foo baz,\
        \nfoo bang,\
        \nbar baz,\
        \nbar bang {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/042_test_newlines_in_selectors.hrx"
#[test]
fn t042_test_newlines_in_selectors() {
    assert_eq!(
        rsass(
            "foo\
            \nbar {\
            \n  baz\
            \n  bang {a: b}\
            \n\
            \n  bip bop {c: d}}\
            \n"
        )
        .unwrap(),
        "foo bar baz bang {\
        \n  a: b;\
        \n}\
        \nfoo bar bip bop {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/043_test_newlines_in_selectors.hrx"
#[test]
#[ignore] // wrong result
fn t043_test_newlines_in_selectors() {
    assert_eq!(
        rsass(
            "foo, bar\
            \nbaz {\
            \n  bang, bip\
            \n  bop {a: b}}\
            \n"
        )
        .unwrap(),
        "foo bang, foo bip\
        \nbop, bar\
        \nbaz bang, bar\
        \nbaz bip\
        \nbop {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/044_test_trailing_comma_in_selector.hrx"
#[test]
#[ignore] // wrong result
fn t044_test_trailing_comma_in_selector() {
    assert_eq!(
        rsass(
            "#foo #bar,,\
            \n,#baz #boom, {a: b}\
            \n\
            \n#bip #bop, ,, {c: d}\
            \n"
        )
        .unwrap(),
        "#foo #bar,\
        \n#baz #boom {\
        \n  a: b;\
        \n}\
        \n#bip #bop {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/045_test_parent_selectors.hrx"
#[test]
fn t045_test_parent_selectors() {
    assert_eq!(
        rsass(
            "foo {\
            \n  &:hover {a: b}\
            \n  bar &.baz {c: d}}\
            \n"
        )
        .unwrap(),
        "foo:hover {\
        \n  a: b;\
        \n}\
        \nbar foo.baz {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/046_test_parent_selector_with_subject.hrx"
#[test]
#[ignore] // unexepected error
fn t046_test_parent_selector_with_subject() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar &.baz! .bip {a: b}}\
            \n\
            \nfoo bar {\
            \n  bar &.baz! .bip {c: d}}\
            \n"
        )
        .unwrap(),
        "bar foo.baz! .bip {\
        \n  a: b;\
        \n}\
        \nbar foo bar.baz! .bip {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/047_test_unknown_directive_bubbling.hrx"
#[test]
fn t047_test_unknown_directive_bubbling() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  @fblthp {\
            \n    .bar {a: b}\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@fblthp {\
        \n  .foo .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/048_test_namespace_properties.hrx"
#[test]
fn t048_test_namespace_properties() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: baz;\
            \n  bang: {\
            \n    bip: 1px;\
            \n    bop: bar;}}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n  bang-bip: 1px;\
        \n  bang-bop: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/049_test_several_namespace_properties.hrx"
#[test]
fn t049_test_several_namespace_properties() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: baz;\
            \n  bang: {\
            \n    bip: 1px;\
            \n    bop: bar;}\
            \n  buzz: {\
            \n    fram: \"foo\";\
            \n    frum: moo;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n  bang-bip: 1px;\
        \n  bang-bop: bar;\
        \n  buzz-fram: \"foo\";\
        \n  buzz-frum: moo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/050_test_nested_namespace_properties.hrx"
#[test]
fn t050_test_nested_namespace_properties() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: baz;\
            \n  bang: {\
            \n    bip: 1px;\
            \n    bop: bar;\
            \n    blat:{baf:bort}}}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n  bang-bip: 1px;\
        \n  bang-bop: bar;\
        \n  bang-blat-baf: bort;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/051_test_namespace_properties_with_value.hrx"
#[test]
fn t051_test_namespace_properties_with_value() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: baz {\
            \n    bip: bop;\
            \n    bing: bop; }}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n  bar-bip: bop;\
        \n  bar-bing: bop;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/052_test_namespace_properties_with_script_value.hrx"
#[test]
fn t052_test_namespace_properties_with_script_value() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: baz + bang {\
            \n    bip: bop;\
            \n    bing: bop; }}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: bazbang;\
        \n  bar-bip: bop;\
        \n  bar-bing: bop;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/053_test_no_namespace_properties_without_space.hrx"
#[test]
fn t053_test_no_namespace_properties_without_space() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar:baz {\
            \n    bip: bop }}\
            \n"
        )
        .unwrap(),
        "foo bar:baz {\
        \n  bip: bop;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/054_test_basic_mixins.hrx"
#[test]
fn t054_test_basic_mixins() {
    assert_eq!(
        rsass(
            "@mixin foo {\
            \n  .foo {a: b}}\
            \n\
            \n@include foo;\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/055_test_basic_mixins.hrx"
#[test]
fn t055_test_basic_mixins() {
    assert_eq!(
        rsass(
            "@mixin foo {\
            \n  .foo {a: b}}\
            \n\
            \nbar {\
            \n  @include foo;\
            \n  c: d; }\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  c: d;\
        \n}\
        \nbar .foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/056_test_basic_mixins.hrx"
#[test]
fn t056_test_basic_mixins() {
    assert_eq!(
        rsass(
            "@mixin foo {a: b}\
            \n\
            \nbar {\
            \n  @include foo;\
            \n  c: d; }\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: b;\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/057_test_mixins_with_empty_args.hrx"
#[test]
fn t057_test_mixins_with_empty_args() {
    assert_eq!(
        rsass(
            "@mixin foo() {a: b}\
            \n\
            \n.foo {@include foo();}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/058_test_mixins_with_empty_args.hrx"
#[test]
fn t058_test_mixins_with_empty_args() {
    assert_eq!(
        rsass(
            "@mixin foo() {a: b}\
            \n\
            \n.foo {@include foo;}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/059_test_mixins_with_empty_args.hrx"
#[test]
fn t059_test_mixins_with_empty_args() {
    assert_eq!(
        rsass(
            "@mixin foo {a: b}\
            \n\
            \n.foo {@include foo();}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/060_test_mixins_with_args.hrx"
#[test]
fn t060_test_mixins_with_args() {
    assert_eq!(
        rsass(
            "@mixin foo($a) {a: $a}\
            \n\
            \n.foo {@include foo(bar)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/061_test_mixins_with_args.hrx"
#[test]
fn t061_test_mixins_with_args() {
    assert_eq!(
        rsass(
            "@mixin foo($a, $b) {\
            \n  a: $a;\
            \n  b: $b; }\
            \n\
            \n.foo {@include foo(bar, 12px)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: bar;\
        \n  b: 12px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/062_test_basic_function.hrx"
#[test]
fn t062_test_basic_function() {
    assert_eq!(
        rsass(
            "@function foo() {\
            \n  @return 1 + 2;\
            \n}\
            \n\
            \nbar {\
            \n  a: foo();\
            \n}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: 3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/063_test_function_args.hrx"
#[test]
fn t063_test_function_args() {
    assert_eq!(
        rsass(
            "@function plus($var1, $var2) {\
            \n  @return $var1 + $var2;\
            \n}\
            \n\
            \nbar {\
            \n  a: plus(1, 2);\
            \n}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: 3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/064_test_mixin_var_args.hrx"
#[test]
fn t064_test_mixin_var_args() {
    assert_eq!(
        rsass(
            "@mixin foo($a, $b...) {\
            \n  a: $a;\
            \n  b: $b;\
            \n}\
            \n\
            \n.foo {@include foo(1, 2, 3, 4)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  b: 2, 3, 4;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/065_test_mixin_empty_var_args.hrx"
#[test]
fn t065_test_mixin_empty_var_args() {
    assert_eq!(
        rsass(
            "@mixin foo($a, $b...) {\
            \n  a: $a;\
            \n  b: length($b);\
            \n}\
            \n\
            \n.foo {@include foo(1)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  b: 0;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/066_test_mixin_var_args_act_like_list.hrx"
#[test]
fn t066_test_mixin_var_args_act_like_list() {
    assert_eq!(
        rsass(
            "@mixin foo($a, $b...) {\
            \n  a: length($b);\
            \n  b: nth($b, 2);\
            \n}\
            \n\
            \n.foo {@include foo(1, 2, 3, 4)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 3;\
        \n  b: 3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/067_test_mixin_splat_args.hrx"
#[test]
#[ignore] // wrong result
fn t067_test_mixin_splat_args() {
    assert_eq!(
        rsass(
            "@mixin foo($a, $b, $c, $d) {\
            \n  a: $a;\
            \n  b: $b;\
            \n  c: $c;\
            \n  d: $d;\
            \n}\
            \n\
            \n$list: 2, 3, 4;\
            \n.foo {@include foo(1, $list...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  b: 2;\
        \n  c: 3;\
        \n  d: 4;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/068_test_mixin_splat_expression.hrx"
#[test]
#[ignore] // wrong result
fn t068_test_mixin_splat_expression() {
    assert_eq!(
        rsass(
            "@mixin foo($a, $b, $c, $d) {\
            \n  a: $a;\
            \n  b: $b;\
            \n  c: $c;\
            \n  d: $d;\
            \n}\
            \n\
            \n.foo {@include foo(1, (2, 3, 4)...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  b: 2;\
        \n  c: 3;\
        \n  d: 4;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/069_test_mixin_splat_args_with_var_args.hrx"
#[test]
#[ignore] // wrong result
fn t069_test_mixin_splat_args_with_var_args() {
    assert_eq!(
        rsass(
            "@mixin foo($a, $b...) {\
            \n  a: $a;\
            \n  b: $b;\
            \n}\
            \n\
            \n$list: 2, 3, 4;\
            \n.foo {@include foo(1, $list...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  b: 2, 3, 4;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/070_test_mixin_splat_args_with_var_args_and_normal_args.hrx"
#[test]
#[ignore] // wrong result
fn t070_test_mixin_splat_args_with_var_args_and_normal_args() {
    assert_eq!(
        rsass(
            "@mixin foo($a, $b, $c...) {\
            \n  a: $a;\
            \n  b: $b;\
            \n  c: $c;\
            \n}\
            \n\
            \n$list: 2, 3, 4;\
            \n.foo {@include foo(1, $list...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  b: 2;\
        \n  c: 3, 4;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/071_test_mixin_splat_args_with_var_args_preserves_separator.hrx"
#[test]
#[ignore] // wrong result
fn t071_test_mixin_splat_args_with_var_args_preserves_separator() {
    assert_eq!(
        rsass(
            "@mixin foo($a, $b...) {\
            \n  a: $a;\
            \n  b: $b;\
            \n}\
            \n\
            \n$list: 3 4 5;\
            \n.foo {@include foo(1, 2, $list...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  b: 2 3 4 5;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/072_test_mixin_var_and_splat_args_pass_through_keywords.hrx"
#[test]
#[ignore] // wrong result
fn t072_test_mixin_var_and_splat_args_pass_through_keywords() {
    assert_eq!(
        rsass(
            "@mixin foo($a...) {\
            \n  @include bar($a...);\
            \n}\
            \n\
            \n@mixin bar($b, $c, $a) {\
            \n  a: $a;\
            \n  b: $b;\
            \n  c: $c;\
            \n}\
            \n\
            \n.foo {@include foo(1, $c: 2, $a: 3)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 3;\
        \n  b: 1;\
        \n  c: 2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/078_test_mixin_list_of_pairs_splat_treated_as_list.hrx"
#[test]
#[ignore] // wrong result
fn t078_test_mixin_list_of_pairs_splat_treated_as_list() {
    assert_eq!(
        rsass(
            "@mixin foo($a, $b, $c) {\
            \n  a: $a;\
            \n  b: $b;\
            \n  c: $c;\
            \n}\
            \n\
            \n.foo {\
            \n  @include foo((a 1, b 2, c 3)...);\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: a 1;\
        \n  b: b 2;\
        \n  c: c 3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/083_test_function_var_args.hrx"
#[test]
fn t083_test_function_var_args() {
    assert_eq!(
        rsass(
            "@function foo($a, $b...) {\
            \n  @return \"a: #{$a}, b: #{$b}\";\
            \n}\
            \n\
            \n.foo {val: foo(1, 2, 3, 4)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: 1, b: 2, 3, 4\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/084_test_function_empty_var_args.hrx"
#[test]
fn t084_test_function_empty_var_args() {
    assert_eq!(
        rsass(
            "@function foo($a, $b...) {\
            \n  @return \"a: #{$a}, b: #{length($b)}\";\
            \n}\
            \n\
            \n.foo {val: foo(1)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: 1, b: 0\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/085_test_function_var_args_act_like_list.hrx"
#[test]
fn t085_test_function_var_args_act_like_list() {
    assert_eq!(
        rsass(
            "@function foo($a, $b...) {\
            \n  @return \"a: #{length($b)}, b: #{nth($b, 2)}\";\
            \n}\
            \n\
            \n.foo {val: foo(1, 2, 3, 4)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: 3, b: 3\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/086_test_function_splat_args.hrx"
#[test]
#[ignore] // wrong result
fn t086_test_function_splat_args() {
    assert_eq!(
        rsass(
            "@function foo($a, $b, $c, $d) {\
            \n  @return \"a: #{$a}, b: #{$b}, c: #{$c}, d: #{$d}\";\
            \n}\
            \n\
            \n$list: 2, 3, 4;\
            \n.foo {val: foo(1, $list...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: 1, b: 2, c: 3, d: 4\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/087_test_function_splat_expression.hrx"
#[test]
#[ignore] // wrong result
fn t087_test_function_splat_expression() {
    assert_eq!(
        rsass(
            "@function foo($a, $b, $c, $d) {\
            \n  @return \"a: #{$a}, b: #{$b}, c: #{$c}, d: #{$d}\";\
            \n}\
            \n\
            \n.foo {val: foo(1, (2, 3, 4)...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: 1, b: 2, c: 3, d: 4\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/088_test_function_splat_args_with_var_args.hrx"
#[test]
#[ignore] // wrong result
fn t088_test_function_splat_args_with_var_args() {
    assert_eq!(
        rsass(
            "@function foo($a, $b...) {\
            \n  @return \"a: #{$a}, b: #{$b}\";\
            \n}\
            \n\
            \n$list: 2, 3, 4;\
            \n.foo {val: foo(1, $list...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: 1, b: 2, 3, 4\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/089_test_function_splat_args_with_var_args_and_normal_args.hrx"
#[test]
#[ignore] // wrong result
fn t089_test_function_splat_args_with_var_args_and_normal_args() {
    assert_eq!(
        rsass(
            "@function foo($a, $b, $c...) {\
            \n  @return \"a: #{$a}, b: #{$b}, c: #{$c}\";\
            \n}\
            \n\
            \n$list: 2, 3, 4;\
            \n.foo {val: foo(1, $list...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: 1, b: 2, c: 3, 4\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/090_test_function_splat_args_with_var_args_preserves_separator.hrx"
#[test]
#[ignore] // wrong result
fn t090_test_function_splat_args_with_var_args_preserves_separator() {
    assert_eq!(
        rsass(
            "@function foo($a, $b...) {\
            \n  @return \"a: #{$a}, b: #{$b}\";\
            \n}\
            \n\
            \n$list: 3 4 5;\
            \n.foo {val: foo(1, 2, $list...)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: 1, b: 2 3 4 5\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/091_test_function_var_and_splat_args_pass_through_keywords.hrx"
#[test]
#[ignore] // wrong result
fn t091_test_function_var_and_splat_args_pass_through_keywords() {
    assert_eq!(
        rsass(
            "@function foo($a...) {\
            \n  @return bar($a...);\
            \n}\
            \n\
            \n@function bar($b, $c, $a) {\
            \n  @return \"a: #{$a}, b: #{$b}, c: #{$c}\";\
            \n}\
            \n\
            \n.foo {val: foo(1, $c: 2, $a: 3)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: 3, b: 1, c: 2\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/098_test_function_list_of_pairs_splat_treated_as_list.hrx"
#[test]
#[ignore] // wrong result
fn t098_test_function_list_of_pairs_splat_treated_as_list() {
    assert_eq!(
        rsass(
            "@function foo($a, $b, $c) {\
            \n  @return \"a: #{$a}, b: #{$b}, c: #{$c}\";\
            \n}\
            \n\
            \n.foo {\
            \n  val: foo((a 1, b 2, c 3)...);\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: \"a: a 1, b: b 2, c: c 3\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/103_test_function_var_args_passed_to_native.hrx"
#[test]
#[ignore] // unexepected error
fn t103_test_function_var_args_passed_to_native() {
    assert_eq!(
        rsass(
            "@function foo($args...) {\
            \n  @return adjust-color($args...);\
            \n}\
            \n\
            \n.foo {val: foo(#102030, $blue: 5)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  val: #102035;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/104_test_basic_selector_interpolation.hrx"
#[test]
fn t104_test_basic_selector_interpolation() {
    assert_eq!(
        rsass(
            "foo#{1 + 2} baz {a: b}\
            \n"
        )
        .unwrap(),
        "foo3 baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/105_test_basic_selector_interpolation.hrx"
#[test]
fn t105_test_basic_selector_interpolation() {
    assert_eq!(
        rsass(
            "foo#{\".bar\"} baz {a: b}\
            \n"
        )
        .unwrap(),
        "foo.bar baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/106_test_basic_selector_interpolation.hrx"
#[test]
fn t106_test_basic_selector_interpolation() {
    assert_eq!(
        rsass(
            "#{\"foo\"}.bar baz {a: b}\
            \n"
        )
        .unwrap(),
        "foo.bar baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/107_test_selector_only_interpolation.hrx"
#[test]
fn t107_test_selector_only_interpolation() {
    assert_eq!(
        rsass(
            "#{\"foo\" + \" bar\"} {a: b}\
            \n"
        )
        .unwrap(),
        "foo bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/108_test_selector_interpolation_before_element_name.hrx"
#[test]
fn t108_test_selector_interpolation_before_element_name() {
    assert_eq!(
        rsass(
            "#{\"foo\" + \" bar\"}baz {a: b}\
            \n"
        )
        .unwrap(),
        "foo barbaz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/109_test_selector_interpolation_in_string.hrx"
#[test]
fn t109_test_selector_interpolation_in_string() {
    assert_eq!(
        rsass(
            "foo[val=\"bar #{\"foo\" + \" bar\"} baz\"] {a: b}\
            \n"
        )
        .unwrap(),
        "foo[val=\"bar foo bar baz\"] {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/110_test_selector_interpolation_in_pseudoclass.hrx"
#[test]
fn t110_test_selector_interpolation_in_pseudoclass() {
    assert_eq!(
        rsass(
            "foo:nth-child(#{5 + \"n\"}) {a: b}\
            \n"
        )
        .unwrap(),
        "foo:nth-child(5n) {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/111_test_selector_interpolation_at_class_begininng.hrx"
#[test]
fn t111_test_selector_interpolation_at_class_begininng() {
    assert_eq!(
        rsass(
            "$zzz: zzz;\
            \n.#{$zzz} { a: b; }\
            \n"
        )
        .unwrap(),
        ".zzz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/112_test_selector_interpolation_at_id_begininng.hrx"
#[test]
fn t112_test_selector_interpolation_at_id_begininng() {
    assert_eq!(
        rsass(
            "$zzz: zzz;\
            \n##{$zzz} { a: b; }\
            \n"
        )
        .unwrap(),
        "#zzz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/113_test_selector_interpolation_at_pseudo_begininng.hrx"
#[test]
fn t113_test_selector_interpolation_at_pseudo_begininng() {
    assert_eq!(
        rsass(
            "$zzz: zzz;\
            \n:#{$zzz}::#{$zzz} { a: b; }\
            \n"
        )
        .unwrap(),
        ":zzz::zzz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/114_test_selector_interpolation_at_attr_beginning.hrx"
#[test]
fn t114_test_selector_interpolation_at_attr_beginning() {
    assert_eq!(
        rsass(
            "$zzz: zzz;\
            \n[#{$zzz}=foo] { a: b; }\
            \n"
        )
        .unwrap(),
        "[zzz=foo] {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/115_test_selector_interpolation_at_attr_end.hrx"
#[test]
fn t115_test_selector_interpolation_at_attr_end() {
    assert_eq!(
        rsass(
            "$zzz: zzz;\
            \n[foo=#{$zzz}] { a: b; }\
            \n"
        )
        .unwrap(),
        "[foo=zzz] {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/116_test_selector_interpolation_at_dashes.hrx"
#[test]
fn t116_test_selector_interpolation_at_dashes() {
    assert_eq!(
        rsass(
            "$a : a;\
            \n$b : b;\
            \ndiv { -foo-#{$a}-#{$b}-foo: foo }\
            \n"
        )
        .unwrap(),
        "div {\
        \n  -foo-a-b-foo: foo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/118_test_parent_selector_with_parent_and_subject.hrx"
#[test]
#[ignore] // wrong result
fn t118_test_parent_selector_with_parent_and_subject() {
    assert_eq!(
        rsass(
            "$subject: \"!\";\
            \nfoo {\
            \n  bar &.baz#{$subject} .bip {c: d}}\
            \n"
        )
        .unwrap(),
        "bar foo.baz! .bip {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/119_test_basic_prop_name_interpolation.hrx"
#[test]
fn t119_test_basic_prop_name_interpolation() {
    assert_eq!(
        rsass(
            "foo {bar#{\"baz\" + \"bang\"}: blip}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  barbazbang: blip;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/120_test_basic_prop_name_interpolation.hrx"
#[test]
fn t120_test_basic_prop_name_interpolation() {
    assert_eq!(
        rsass(
            "foo {bar#{1 + 2}: blip}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar3: blip;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/121_test_prop_name_only_interpolation.hrx"
#[test]
fn t121_test_prop_name_only_interpolation() {
    assert_eq!(
        rsass(
            "foo {#{\"baz\" + \"bang\"}: blip}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bazbang: blip;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/122_test_directive_interpolation.hrx"
#[test]
fn t122_test_directive_interpolation() {
    assert_eq!(
        rsass(
            "$baz: 12;\
            \n@foo bar#{$baz} qux {a: b}\
            \n"
        )
        .unwrap(),
        "@foo bar12 qux {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/128_test_supports_with_expressions.hrx"
#[test]
fn t128_test_supports_with_expressions() {
    assert_eq!(
        rsass(
            "$query: \"(feature1: val)\";\
            \n$feature: feature2;\
            \n$val: val;\
            \n@supports (#{$query} and ($feature: $val)) or (not ($feature + 3: $val + 4)) {\
            \n  foo {a: b}\
            \n}\
            \n"
        )
        .unwrap(),
        "@supports ((feature1: val) and (feature2: val)) or (not (feature23: val4)) {\
        \n  foo {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/129_test_supports_bubbling.hrx"
#[test]
#[ignore] // wrong result
fn t129_test_supports_bubbling() {
    assert_eq!(
        rsass(
            "a {\
            \n  @supports (foo: bar) {\
            \n    b: c;\
            \n    @supports (baz: bang) {\
            \n      d: e;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@supports (foo: bar) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n  @supports (baz: bang) {\
        \n    a {\
        \n      d: e;\
        \n    }\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/130_test_random_directive_interpolation.hrx"
#[test]
#[ignore] // wrong result
fn t130_test_random_directive_interpolation() {
    assert_eq!(
        rsass(
            "$domain: \"sass-lang.com\";\
            \n@foo url(https://#{$domain}/),\
            \n     #{domain($domain)},\
            \n     \"foo#{\'ba\' + \'r\'}baz\",\
            \n     foo#{\'ba\' + \'r\'}baz {\
            \n  .foo {a: b}\
            \n}\
            \n"
        )
        .unwrap(),
        "@foo url(https://sass-lang.com/),\
        \n     domain(\"sass-lang.com\"),\
        \n     \"foobarbaz\",\
        \n     foobarbaz {\
        \n  .foo {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/131_test_nested_mixin_def.hrx"
#[test]
fn t131_test_nested_mixin_def() {
    assert_eq!(
        rsass(
            "foo {\
            \n  @mixin bar {a: b}\
            \n  @include bar; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/132_test_nested_mixin_shadow.hrx"
#[test]
fn t132_test_nested_mixin_shadow() {
    assert_eq!(
        rsass(
            "@mixin bar {a: b}\
            \n\
            \nfoo {\
            \n  @mixin bar {c: d}\
            \n  @include bar;\
            \n}\
            \n\
            \nbaz {@include bar}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  c: d;\
        \n}\
        \nbaz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/133_test_nested_function_def.hrx"
#[test]
fn t133_test_nested_function_def() {
    assert_eq!(
        rsass(
            "foo {\
            \n  @function foo() {@return 1}\
            \n  a: foo(); }\
            \n\
            \nbar {b: foo()}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 1;\
        \n}\
        \nbar {\
        \n  b: foo();\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/134_test_nested_function_shadow.hrx"
#[test]
fn t134_test_nested_function_shadow() {
    assert_eq!(
        rsass(
            "@function foo() {@return 1}\
            \n\
            \nfoo {\
            \n  @function foo() {@return 2}\
            \n  a: foo();\
            \n}\
            \n\
            \nbaz {b: foo()}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 2;\
        \n}\
        \nbaz {\
        \n  b: 1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/171_test_loud_comment_in_compressed_mode.hrx"
#[test]
fn t171_test_loud_comment_in_compressed_mode() {
    assert_eq!(
        rsass(
            "/*! foo */\
            \n"
        )
        .unwrap(),
        "/*! foo */\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/172_test_parsing_decimals_followed_by_comments_doesnt_take_forever.hrx"
#[test]
#[ignore] // wrong result
fn t172_test_parsing_decimals_followed_by_comments_doesnt_take_forever() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  padding: 4.21052631578947% 4.21052631578947% 5.631578947368421% /**/\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  padding: 4.2105263158% 4.2105263158% 5.6315789474%;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/173_test_parsing_many_numbers_doesnt_take_forever.hrx"
#[test]
fn t173_test_parsing_many_numbers_doesnt_take_forever() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  padding: 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%;\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  padding: 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%, 80% 90%;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/174_test_import_comments_in_imports.hrx"
#[test]
fn t174_test_import_comments_in_imports() {
    assert_eq!(
        rsass(
            "@import \"foo.css\", // this is a comment\
            \n        \"bar.css\", /* this is another comment */\
            \n        \"baz.css\"; // this is a third comment\
            \n"
        )
        .unwrap(),
        "@import \"foo.css\";\
        \n@import \"bar.css\";\
        \n@import \"baz.css\";\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/176_test_newline_selector_rendered_multiple_times.hrx"
#[test]
#[ignore] // wrong result
fn t176_test_newline_selector_rendered_multiple_times() {
    assert_eq!(
        rsass(
            "@for $i from 1 through 2 {\
            \n  form {\
            \n    input,\
            \n    select {\
            \n      color: white;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "form input,\
        \nform select {\
        \n  color: white;\
        \n}\
        \nform input,\
        \nform select {\
        \n  color: white;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/177_test_prop_name_interpolation_after_hyphen.hrx"
#[test]
fn t177_test_prop_name_interpolation_after_hyphen() {
    assert_eq!(
        rsass(
            "a { -#{\"foo\"}-bar: b; }\
            \n"
        )
        .unwrap(),
        "a {\
        \n  -foo-bar: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/178_test_star_plus_and_parent.hrx"
#[test]
fn t178_test_star_plus_and_parent() {
    assert_eq!(
        rsass(
            "foo {*+html & {a: b}}\
            \n"
        )
        .unwrap(),
        "* + html foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/179_test_weird_added_space.hrx"
#[test]
fn t179_test_weird_added_space() {
    assert_eq!(
        rsass(
            "$value : bip;\
            \n\
            \nfoo {\
            \n  bar: -moz-#{$value};\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: -moz-bip;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/180_test_interpolation_with_bracket_on_next_line.hrx"
#[test]
fn t180_test_interpolation_with_bracket_on_next_line() {
    assert_eq!(
        rsass(
            "a.#{\"foo\"} b\
            \n{color: red}\
            \n"
        )
        .unwrap(),
        "a.foo b {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/181_test_interpolation.hrx"
#[test]
fn t181_test_interpolation() {
    assert_eq!(
        rsass(
            "$bar : \"#foo\";\
            \nul li#{$bar} a span.label { foo: bar; }\
            \n"
        )
        .unwrap(),
        "ul li#foo a span.label {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/182_test_mixin_with_keyword_args.hrx"
#[test]
fn t182_test_mixin_with_keyword_args() {
    assert_eq!(
        rsass(
            "@mixin a-mixin($required, $arg1: default-val1, $arg2: default-val2) {\
            \n  required: $required;\
            \n  arg1: $arg1;\
            \n  arg2: $arg2;\
            \n}\
            \n.mixed { @include a-mixin(foo, $arg2: non-default-val2); }\
            \n"
        )
        .unwrap(),
        ".mixed {\
        \n  required: foo;\
        \n  arg1: default-val1;\
        \n  arg2: non-default-val2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/183_test_passing_required_args_as_a_keyword_arg.hrx"
#[test]
fn t183_test_passing_required_args_as_a_keyword_arg() {
    assert_eq!(
        rsass(
            "@mixin a-mixin($required, $arg1: default-val1, $arg2: default-val2) {\
            \n  required: $required;\
            \n  arg1: $arg1;\
            \n  arg2: $arg2; }\
            \n.mixed { @include a-mixin($required: foo); }\
            \n"
        )
        .unwrap(),
        ".mixed {\
        \n  required: foo;\
        \n  arg1: default-val1;\
        \n  arg2: default-val2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/184_test_passing_all_as_keyword_args_in_opposite_order.hrx"
#[test]
fn t184_test_passing_all_as_keyword_args_in_opposite_order() {
    assert_eq!(
        rsass(
            "@mixin a-mixin($required, $arg1: default-val1, $arg2: default-val2) {\
            \n  required: $required;\
            \n  arg1: $arg1;\
            \n  arg2: $arg2; }\
            \n.mixed { @include a-mixin($arg2: non-default-val2, $arg1: non-default-val1, $required: foo); }\
            \n"
        )
        .unwrap(),
        ".mixed {\
        \n  required: foo;\
        \n  arg1: non-default-val1;\
        \n  arg2: non-default-val2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/185_test_keyword_args_in_functions.hrx"
#[test]
fn t185_test_keyword_args_in_functions() {
    assert_eq!(
        rsass(
            ".keyed { color: rgba($color: #a7c, $alpha: 0.4) }\
            \n"
        )
        .unwrap(),
        ".keyed {\
        \n  color: rgba(170, 119, 204, 0.4);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/186_test_newlines_removed_from_selectors_when_compressed.hrx"
#[test]
#[ignore] // wrong result
fn t186_test_newlines_removed_from_selectors_when_compressed() {
    assert_eq!(
        rsass(
            "a\
            \n, b {\
            \n  z & {\
            \n    display: block;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "z a,\
        \nz b {\
        \n  display: block;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/187_test_multiline_var.hrx"
#[test]
fn t187_test_multiline_var() {
    assert_eq!(
        rsass(
            "foo {\
            \n  $var1: 1 +\
            \n    2;\
            \n  $var2: true and\
            \n    false;\
            \n  $var3: a b\
            \n    c;\
            \n  a: $var1;\
            \n  b: $var2;\
            \n  c: $var3; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 3;\
        \n  b: false;\
        \n  c: a b c;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/188_test_mixin_content.hrx"
#[test]
#[ignore] // wrong result
fn t188_test_mixin_content() {
    assert_eq!(
        rsass(
            "$color: blue;\
            \n@mixin context($class, $color: red) {\
            \n  .#{$class} {\
            \n    background-color: $color;\
            \n    @content;\
            \n    border-color: $color;\
            \n  }\
            \n}\
            \n@include context(parent) {\
            \n  @include context(child, $color: yellow) {\
            \n    color: $color;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent {\
        \n  background-color: red;\
        \n  border-color: red;\
        \n}\
        \n.parent .child {\
        \n  background-color: yellow;\
        \n  color: blue;\
        \n  border-color: yellow;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/189_test_empty_content.hrx"
#[test]
fn t189_test_empty_content() {
    assert_eq!(
        rsass(
            "@mixin foo { @content }\
            \na { b: c; @include foo {} }\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/190_test_options_passed_to_script.hrx"
#[test]
fn t190_test_options_passed_to_script() {
    assert_eq!(
        rsass(
            "foo {color: darken(black, 10%)}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  color: black;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss-tests/191_test_extend_in_media_in_rule.hrx"
#[test]
#[ignore] // wrong result
fn t191_test_extend_in_media_in_rule() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  @media screen {\
            \n    @extend %bar;\
            \n  }\
            \n}\
            \n\
            \n@media screen {\
            \n  %bar {\
            \n    a: b;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  .foo {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}
