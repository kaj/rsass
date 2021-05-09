//! Tests auto-converted from "sass-spec/spec/libsass/http_import.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
