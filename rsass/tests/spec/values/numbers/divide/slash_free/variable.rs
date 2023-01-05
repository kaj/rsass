//! Tests auto-converted from "sass-spec/spec/values/numbers/divide/slash_free/variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("variable")
        .mock_file(
            "forward_with/_midstream.scss",
            "@forward \"upstream\" with ($a: 1/2);\n",
        )
        .mock_file(
            "forward_with/_upstream.scss",
            "$a: null !default;\nb {c: $a}\n",
        )
        .mock_file("use_with/_other.scss", "$a: null !default;\nb {c: $a}\n")
}

#[test]
fn forward_with() {
    let runner = runner().with_cwd("forward_with");
    assert_eq!(
        runner.ok("@use \"midstream\";\n"),
        "b {\
         \n  c: 0.5;\
         \n}\n"
    );
}
#[test]
fn local() {
    let runner = runner().with_cwd("local");
    assert_eq!(
        runner.ok("$a: 1/2;\
             \nb {c: $a}\n"),
        "b {\
         \n  c: 0.5;\
         \n}\n"
    );
}
#[test]
fn use_with() {
    let runner = runner().with_cwd("use_with");
    assert_eq!(
        runner.ok("@use \"other\" with ($a: 1/2);\n"),
        "b {\
         \n  c: 0.5;\
         \n}\n"
    );
}
