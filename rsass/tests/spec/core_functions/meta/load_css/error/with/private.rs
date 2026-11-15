//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/error/with/private.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("private")
        .mock_file("different/_other.scss", "$_a: c !default;\nd {e: $_a}\n")
        .mock_file(
            "matching/dash/_other.scss",
            "$-a: c !default;\nd {e: $-a}\n",
        )
        .mock_file(
            "matching/underscore/_other.scss",
            "$_a: c !default;\nd {e: $_a}\n",
        )
}

#[test]
fn different() {
    let runner = runner().with_cwd("different");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (\"-a\": b));\n"),
        "d {\
         \n  e: b;\
         \n}\n"
    );
}
mod matching {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("matching")
    }

    #[test]
    fn dash() {
        let runner = runner().with_cwd("dash");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (\"-a\": b));\n"),
            "d {\
         \n  e: b;\
         \n}\n"
        );
    }
    #[test]
    fn underscore() {
        let runner = runner().with_cwd("underscore");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (\"_a\": b));\n"),
            "d {\
         \n  e: b;\
         \n}\n"
        );
    }
}
