//! Tests auto-converted from "sass-spec/spec/libsass/http_import.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("http_import")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@import \"http://fonts.googleapis.com/css?family=Droid+Sans\";"
        ),
        "@import \"http://fonts.googleapis.com/css?family=Droid+Sans\";\n"
    );
}
