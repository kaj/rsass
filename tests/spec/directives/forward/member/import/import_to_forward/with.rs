//! Tests auto-converted from "sass-spec/spec/directives/forward/member/import/import_to_forward/with.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file(
            "default/_midstream.scss",
            "@forward \"upstream\" with ($c: midstream);\n",
        )
        .mock_file(
            "default/_upstream.scss",
            "$c: upstream !default;\n\n@function get-c() {@return $c}\n",
        )
        .mock_file(
            "non_overridable/_midstream.scss",
            "@forward \"upstream\" with ($a: midstream);\n",
        )
        .mock_file(
            "non_overridable/_upstream.scss",
            "$a: upstream !default;\n\n@function get-a() {@return $a}\n",
        )
        .mock_file(
            "overridden/_midstream.scss",
            "@forward \"upstream\" with ($a: midstream !default);\n",
        )
        .mock_file(
            "overridden/_upstream.scss",
            "$a: upstream !default;\n\n@function get-a() {@return $a}\n",
        )
}

#[test]
fn default() {
    let runner = runner().with_cwd("default");
    assert_eq!(
        runner.ok("@import \"midstream\";\n\
             \na {b: get-c()};\n"),
        "a {\
         \n  b: midstream;\
         \n}\n"
    );
}
#[test]
fn non_overridable() {
    let runner = runner().with_cwd("non_overridable");
    assert_eq!(
        runner.ok("$a: input;\n\
             \n@import \"midstream\";\n\
             \nb {c: get-a()};\n"),
        "b {\
         \n  c: midstream;\
         \n}\n"
    );
}
#[test]
fn overridden() {
    let runner = runner().with_cwd("overridden");
    assert_eq!(
        runner.ok("$a: input;\n\
             \n@import \"midstream\";\n\
             \nb {c: get-a()};\n"),
        "b {\
         \n  c: input;\
         \n}\n"
    );
}
