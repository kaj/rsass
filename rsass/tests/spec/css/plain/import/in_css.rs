//! Tests auto-converted from "sass-spec/spec/css/plain/import/in_css.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("in_css")
        .mock_file("string/plain.css", "@import \"whatever\";\n")
        .mock_file("url/quoted/plain.css", "@import url(\"whatever\");\n")
        .mock_file("url/unquoted/plain.css", "@import url(whatever);\n")
}

#[test]
fn string() {
    let runner = runner().with_cwd("string");
    assert_eq!(runner.ok("@use \"plain\";\n"), "@import \"whatever\";\n");
}
mod url {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("url")
    }

    #[test]
    fn quoted() {
        let runner = runner().with_cwd("quoted");
        assert_eq!(
            runner.ok("@use \"plain\";\n"),
            "@import url(\"whatever\");\n"
        );
    }
    #[test]
    fn unquoted() {
        let runner = runner().with_cwd("unquoted");
        assert_eq!(
            runner.ok("@use \"plain\";\n"),
            "@import url(whatever);\n"
        );
    }
}
