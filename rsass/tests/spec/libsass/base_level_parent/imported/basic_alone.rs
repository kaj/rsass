//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/basic-alone.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("basic-alone")
        .mock_file("include.scss", "& {\n  foo {\n    bar: baz;\n  }\n}\n")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@import \"include.scss\";"),
        "& foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
