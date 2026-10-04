//! Tests auto-converted from "sass-spec/spec/core_functions/meta/load/error/load.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("load")
        .mock_file(
            "loop/_other.scss",
            "@use \"sass:meta\";\nc {d: meta.load(\"input\")}\n",
        )
        .mock_file(
            "loop/input.scss",
            "@use \"sass:meta\";\n$_: meta.load(\"other\");\n",
        )
}

#[test]
#[ignore] // wrong error
fn test_loop() {
    let runner = runner().with_cwd("loop");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n$_: meta.load(\"other\");\n"
        ),
        "Error: Module loop: input.scss is already being loaded.\
         \n  ,\
         \n2 | c {d: meta.load(\"input\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  _other.scss 2:7  load()\
         \n  input.scss 2:5   root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn missing() {
    let runner = runner().with_cwd("missing");
    assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n$_: meta.load(\"other\");\n"
        ),
        "Error: Can\'t find stylesheet to import.\
         \n  ,\
         \n2 | $_: meta.load(\"other\");\
         \n  |     ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
    );
}
