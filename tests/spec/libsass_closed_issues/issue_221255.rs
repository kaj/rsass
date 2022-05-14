//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_221255.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_221255")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "\'#{)\'{"
        ),
        "Error: Invalid CSS after \"\'#{\": expected expression (e.g. 1px, bold), was \")\'{\"\
         \n        on line 1 of input.scss\
         \n  Use --trace for backtrace.",
    );
}
