//! Tests auto-converted from "sass-spec/spec/directives/forward/with/through_import.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("through_import")
        .mock_file(
            "direct/_downstream.scss",
            "@forward \"midstream\" with ($a: configured);\n",
        )
        .mock_file("direct/_midstream.scss", "@import \"upstream\";\n")
        .mock_file(
            "direct/_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "transitive/_forwarded.scss",
            "@import \"imported_downstream\";\n",
        )
        .mock_file(
            "transitive/_imported_downstream.scss",
            "@import \"imported_upstream\";\n",
        )
        .mock_file(
            "transitive/_imported_upstream.scss",
            "$a: original !default;\nb {c: $a}\n",
        )
        .mock_file(
            "transitive/_used.scss",
            "@forward \"forwarded\" with ($a: configured);\n",
        )
}

#[test]
fn direct() {
    let runner = runner().with_cwd("direct");
    assert_eq!(
        runner.ok("@use \"downstream\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
fn transitive() {
    let runner = runner().with_cwd("transitive");
    assert_eq!(
        runner.ok("@use \"used\";\n"),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
