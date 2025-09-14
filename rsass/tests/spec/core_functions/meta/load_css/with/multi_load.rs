//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load_css/with/multi_load.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("multi_load")
        .mock_file("forward/_loads.scss", "@use \"sass:meta\";\n@include meta.load-css(\"upstream\", $with: (a: configured));\n")
        .mock_file("forward/_midstream.scss", "@forward \"upstream\";\n")
        .mock_file("forward/_upstream.scss", "$a: original !default;\n")
        .mock_file("through_forward/_forwarded.scss", "// This file defines no variables, so it's allowed to be loaded with and without\n// configuration.\n")
        .mock_file("through_forward/_midstream.scss", "@forward \"forwarded\";\n\n$a: default !default;\nb {\n  midstream: $a;\n}\n")
        .mock_file("unused_configuration/double_load/_midstream.scss", "@use \"upstream\";\nb {c: upstream.$a}\n")
        .mock_file("unused_configuration/double_load/_upstream.scss", "$a: original !default;\n")
        .mock_file("unused_configuration/use_and_load/_forwarded.scss", "// This file defines no variables, so it's allowed to be loaded with and without\n// configuration.\n")
        .mock_file("unused_configuration/use_and_load/_midstream.scss", "@forward \"forwarded\";\n\n$a: default !default;\n\nb {\n  midstream: $a;\n}\n")
        .mock_file("use/_midstream.scss", "@use \"upstream\";\nb {c: upstream.$a}\n")
        .mock_file("use/_upstream.scss", "$a: original !default;\n")
}

#[test]
#[ignore] // wrong result
fn forward() {
    let runner = runner().with_cwd("forward");
    assert_eq!(
        runner.ok(
            "// This indirection is necessary so that we can execute `meta.load-css()` before\
             \n// we begin loading `midstream`.\
             \n@use \"loads\";\
             \n@use \"midstream\";\n\
             \nb {c: midstream.$a}\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
#[test]
fn through_forward() {
    let runner = runner().with_cwd("through_forward");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"forwarded\");\
             \n@include meta.load-css(\"midstream\", $with: (a: overridden));\n"
        ),
        "b {\
         \n  midstream: overridden;\
         \n}\n"
    );
}
mod unused_configuration {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("unused_configuration")
    }

    #[test]
    #[ignore] // wrong result
    fn double_load() {
        let runner = runner().with_cwd("double_load");
        assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"upstream\", $with: (a: configured));\n\
             \n// An empty configuration map counts as no configuration.\
             \n@include meta.load-css(\"midstream\", $with: ());\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
    }
    #[test]
    fn use_and_load() {
        let runner = runner().with_cwd("use_and_load");
        assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"forwarded\";\
             \n@include meta.load-css(\"midstream\", $with: (a: overridden));\n"
        ),
        "b {\
         \n  midstream: overridden;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // wrong result
fn test_use() {
    let runner = runner().with_cwd("use");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@include meta.load-css(\"upstream\", $with: (a: configured));\n\
             \n// We have to load this dynamically, because we can\'t have a `@use` after an\
             \n// `@include`.\
             \n@include meta.load-css(\"midstream\");\n"
        ),
        "b {\
         \n  c: configured;\
         \n}\n"
    );
}
