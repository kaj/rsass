//! Tests auto-converted from "sass-spec/spec/directives/forward/css.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("css")
        .mock_file(
            "forward_into_import/_forwarded.scss",
            "@import \"imported\";\n\nin-forwarded {a: b}\n",
        )
        .mock_file(
            "forward_into_import/_imported.scss",
            "in-imported {a: b}\n",
        )
        .mock_file("forward_only/_other.scss", "a {b: c}\n")
        .mock_file("once/forward_and_use/_other.scss", "a {b: c}\n")
        .mock_file("once/multiple_forwards/_other.scss", "a {b: c}\n")
        .mock_file("order/_other1.scss", "a {file: other1}\n")
        .mock_file("order/_other2.scss", "a {file: other2}\n")
        .mock_file("order/_other3.scss", "a {file: other3}\n")
}

#[test]
fn forward_into_import() {
    let runner = runner().with_cwd("forward_into_import");
    assert_eq!(
        runner.ok("@forward \"forwarded\";\n\
             \nin-input {a: b}\n"),
        "in-imported {\
         \n  a: b;\
         \n}\
         \nin-forwarded {\
         \n  a: b;\
         \n}\
         \nin-input {\
         \n  a: b;\
         \n}\n"
    );
}
#[test]
fn forward_only() {
    let runner = runner().with_cwd("forward_only");
    assert_eq!(
        runner.ok("@forward \"other\";\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
mod once {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("once")
    }

    #[test]
    fn forward_and_use() {
        let runner = runner().with_cwd("forward_and_use");
        assert_eq!(
            runner.ok("@forward \"other\";\
             \n@use \"other\";\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    fn multiple_forwards() {
        let runner = runner().with_cwd("multiple_forwards");
        assert_eq!(
            runner.ok("@forward \"other\";\
             \n@forward \"other\";\
             \n@forward \"other\";\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
#[test]
fn order() {
    let runner = runner().with_cwd("order");
    assert_eq!(
        runner.ok("@forward \"other1\";\
             \n@use \"other2\";\
             \n@forward \"other3\";\n\
             \na {file: input}\n"),
        "a {\
         \n  file: other1;\
         \n}\
         \na {\
         \n  file: other2;\
         \n}\
         \na {\
         \n  file: other3;\
         \n}\
         \na {\
         \n  file: input;\
         \n}\n"
    );
}
