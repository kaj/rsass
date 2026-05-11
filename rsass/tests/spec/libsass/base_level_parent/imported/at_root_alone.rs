//! Tests auto-converted from "sass-spec/spec/libsass/base-level-parent/imported/at-root-alone.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("at-root-alone").mock_file(
        "include.scss",
        "@at-root {\n  & {\n    foo {\n      bar: baz;\n    }\n  }\n}",
    )
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
