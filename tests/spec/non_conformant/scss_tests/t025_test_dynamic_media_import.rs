//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/025_test_dynamic_media_import.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("025_test_dynamic_media_import")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$media: print;\
             \n$key: -webkit-min-device-pixel-ratio;\
             \n$value: 20;\
             \n@import \"foo\" #{$media} and ($key + \"-foo\": $value + 5);\n"
        ),
        "@import \"foo\" print and (-webkit-min-device-pixel-ratio-foo: 25);\n"
    );
}
