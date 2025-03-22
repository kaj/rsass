//! Tests auto-converted from "sass-spec/spec/css/plain/slash.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("slash")
        .mock_file("with_intermediate/plain.css", "a {b: 1/2/foo/bar}\n")
        .mock_file(
            "without_intermediate/no_whitespace/plain.css",
            "a {b: 1///bar}\n",
        )
        .mock_file(
            "without_intermediate/whitespace/plain.css",
            "a {b: 1/ / /bar}\n",
        )
}

#[test]
fn with_intermediate() {
    let runner = runner().with_cwd("with_intermediate");
    assert_eq!(
        runner.ok("@use \"plain\";\n"),
        "a {\
         \n  b: 1/2/foo/bar;\
         \n}\n"
    );
}
mod without_intermediate {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("without_intermediate")
    }

    #[test]
    fn no_whitespace() {
        let runner = runner().with_cwd("no_whitespace");
        assert_eq!(
            runner.ok("@use \"plain\";\n"),
            "a {\
         \n  b: 1///bar;\
         \n}\n"
        );
    }
    #[test]
    fn whitespace() {
        let runner = runner().with_cwd("whitespace");
        assert_eq!(
            runner.ok("@use \"plain\";\n"),
            "a {\
         \n  b: 1///bar;\
         \n}\n"
        );
    }
}
