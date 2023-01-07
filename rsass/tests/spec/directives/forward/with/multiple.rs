//! Tests auto-converted from "sass-spec/spec/directives/forward/with/multiple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("multiple")
        .mock_file("default/_midstream.scss", "@forward \"upstream\" with (\n  $a: configured a !default,\n  $b: configured b !default,\n  $c: configured c !default\n);\n")
        .mock_file("default/_upstream.scss", "$a: original a !default;\n$b: original b !default;\n$c: original c !default;\n\nd {\n  a: $a;\n  b: $b;\n  c: $c;\n}\n")
        .mock_file("non_default/_midstream.scss", "@forward \"upstream\" with (\n  $a: configured a,\n  $b: configured b,\n  $c: configured c\n);\n")
        .mock_file("non_default/_upstream.scss", "$a: original a !default;\n$b: original b !default;\n$c: original c !default;\n\nd {\n  a: $a;\n  b: $b;\n  c: $c;\n}\n")
}

#[test]
fn default() {
    let runner = runner().with_cwd("default");
    assert_eq!(
        runner.ok("@use \"midstream\";\n"),
        "d {\
         \n  a: configured a;\
         \n  b: configured b;\
         \n  c: configured c;\
         \n}\n"
    );
}
#[test]
fn non_default() {
    let runner = runner().with_cwd("non_default");
    assert_eq!(
        runner.ok("@use \"midstream\";\n"),
        "d {\
         \n  a: configured a;\
         \n  b: configured b;\
         \n  c: configured c;\
         \n}\n"
    );
}
