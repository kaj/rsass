//! Tests auto-converted from "sass-spec/spec/operators/slash.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("namespaced_variables/other.scss", "$a: 1;\n$b: 2;\n")
}

#[test]
fn namespaced_variables() {
    let runner = runner().with_cwd("namespaced_variables");
    assert_eq!(
        runner.ok("@use \"other\";\
             \na {b: other.$a / other.$b}\n"),
        "a {\
         \n  b: 0.5;\
         \n}\n"
    );
}
