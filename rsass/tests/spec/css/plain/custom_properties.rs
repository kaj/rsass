//! Tests auto-converted from "sass-spec/spec/css/plain/custom_properties.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("custom_properties")
        .mock_file(
            "arbitrary_tokens/plain.css",
            "a {--b: `~@#$%^&*()_-+={[]}|?/><}\n",
        )
        .mock_file("color/plain.css", "a {--b: #ff0000}\n")
        .mock_file("identifier/plain.css", "a {--b: c}\n")
        .mock_file("nested/plain.css", "a {--b: {c: d}}\n")
}

#[test]
#[ignore] // unexepected error
fn arbitrary_tokens() {
    let runner = runner().with_cwd("arbitrary_tokens");
    assert_eq!(
        runner.ok("@import \"plain\";\n"),
        "a {\
         \n  --b: `~@#$%^&*()_-+={[]}|?/><;\
         \n}\n"
    );
}
#[test]
fn color() {
    let runner = runner().with_cwd("color");
    assert_eq!(
        runner.ok("@import \"plain\";\n"),
        "a {\
         \n  --b: #ff0000;\
         \n}\n"
    );
}
#[test]
fn identifier() {
    let runner = runner().with_cwd("identifier");
    assert_eq!(
        runner.ok("@import \"plain\";\n"),
        "a {\
         \n  --b: c;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn nested() {
    let runner = runner().with_cwd("nested");
    assert_eq!(
        runner.ok("@import \"plain\";\n"),
        "a {\
         \n  --b: {c: d};\
         \n}\n"
    );
}
