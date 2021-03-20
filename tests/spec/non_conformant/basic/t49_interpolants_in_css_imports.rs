//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/49_interpolants_in_css_imports.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$google-protocol: \"http\"; // choose http or https\
            \n$google-webfont: \"Open+Sans:400italic,700italic,400,700|Oswald\"; // pull string after ?family= from step 3\
            \n\
            \n@import url(\"#{$google-protocol}://fonts.googleapis.com/css?family=#{$google-webfont}\");"
        )
        .unwrap(),
        "@import url(\"http://fonts.googleapis.com/css?family=Open+Sans:400italic,700italic,400,700|Oswald\");\
        \n"
    );
}
