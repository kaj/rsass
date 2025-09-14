//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with/through_import.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("through_import")
        .mock_file(
            "direct/_imported.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file("direct/_loaded.scss", "@import \"imported\";\n")
        .mock_file("transitive/_loaded.scss", "@import \"midstream\";\n")
        .mock_file("transitive/_midstream.scss", "@import \"upstream\";\n")
        .mock_file(
            "transitive/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
}

#[test]
fn direct() {
    let runner = runner().with_cwd("direct");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: configured));\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
fn transitive() {
    let runner = runner().with_cwd("transitive");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"loaded\", $with: (a: configured));\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
