//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load/shares_state.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("shares_state")
        .mock_file(
            "_other.scss",
            "@use \"shared\";\n\nshared.$b: value set by other;\n",
        )
        .mock_file("_shared.scss", "$b: default value;\n")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@use \"shared\";\
             \n$_: meta.load(\"other\");\n\
             \na {shared-b: shared.$b}\n"),
        "a {\
         \n  shared-b: value set by other;\
         \n}\n"
    );
}
