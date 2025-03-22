//! Tests auto-converted from "sass-spec/spec/libsass/unicode-bom"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unicode-bom")
}

// Ignoring "utf-16-big", not expected to work yet.

// Ignoring "utf-16-little", not expected to work yet.

// From "sass-spec/spec/libsass/unicode-bom/utf-8"
#[test]
fn utf_8() {
    assert_eq!(
        runner().ok("\u{feff}foo { bar: baz; }"),
        "foo {\
         \n  bar: baz;\
         \n}\n"
    );
}
