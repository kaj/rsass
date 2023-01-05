//! Tests auto-converted from "sass-spec/spec/css/plain/import/css_before_index.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("css_before_index")
        .mock_file("index/other.scss", "other {index: true}\n")
        .mock_file("other.css", "other {index: false}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \'other\';\n"),
        "other {\
         \n  index: false;\
         \n}\n"
    );
}
