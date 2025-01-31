//! Tests auto-converted from "sass-spec/spec/css/plain/import/partial_conflict.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("partial_conflict")
        .mock_file("_plain.css", "plain {partial: true}\n")
        .mock_file("plain.css", "plain {partial: false}\n")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@import \"plain\";\n"
        ),
        "DEPRECATION WARNING [import]: Sass @import rules are deprecated and will be removed in Dart Sass 3.0.0.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | @import \"plain\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n    input.scss 1:9  root stylesheet\n\
         \nError: It\'s not clear which file to import. Found:\
         \n  _plain.css\
         \n  plain.css\
         \n  ,\
         \n1 | @import \"plain\";\
         \n  |         ^^^^^^^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
    );
}
