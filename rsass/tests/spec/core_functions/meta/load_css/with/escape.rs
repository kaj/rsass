//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with/escape.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("escape")
        .mock_file(
            "quoted/normalized/_other.scss",
            "$\\$: original !default;\na {b: $\\$}\n",
        )
        .mock_file(
            "unquoted/normalized/_other.scss",
            "$\\$: original !default;\na {b: $\\$}\n",
        )
}

mod quoted {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("quoted")
    }

    #[test]
    #[ignore] // unexepected error
    fn normalized() {
        let runner = runner().with_cwd("normalized");
        assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (\"\\\\$\": override));\n"
        ),
        "a {\
         \n  b: override;\
         \n}\n"
    );
    }
}
mod unquoted {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("unquoted")
    }

    #[test]
    #[ignore] // unexepected error
    fn normalized() {
        let runner = runner().with_cwd("normalized");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (\\$: override));\n"),
            "a {\
         \n  b: override;\
         \n}\n"
        );
    }
}
