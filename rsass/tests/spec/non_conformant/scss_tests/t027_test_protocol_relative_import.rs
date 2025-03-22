//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/027_test_protocol_relative_import.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("027_test_protocol_relative_import")
}

#[test]
fn test() {
    assert_eq!(
        runner()
            .ok("@import \"//fonts.googleapis.com/css?family=Droid+Sans\";"),
        "@import \"//fonts.googleapis.com/css?family=Droid+Sans\";\n"
    );
}
