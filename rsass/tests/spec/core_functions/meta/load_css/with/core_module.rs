//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with/core_module.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("core_module").mock_file(
        "indirect/_other.scss",
        "@use \"sass:color\";\n\n$c: d !default;\n\na {b: $c}\n",
    )
}

#[test]
fn indirect() {
    let runner = runner().with_cwd("indirect");
    assert_eq!(
        runner.ok("// Regression test for sass/dart-sass#838.\
             \n@use \"sass:meta\";\
             \n@include meta.load-css(\"other\", $with: (c: e));\n"),
        "a {\
         \n  b: e;\
         \n}\n"
    );
}
