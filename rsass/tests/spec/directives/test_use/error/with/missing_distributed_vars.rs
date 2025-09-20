//! Tests auto-converted from "sass-spec/spec/directives/use/error/with/missing_distributed_vars.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("missing_distributed_vars")
        .mock_file("multi_use/module/_index.scss", "@forward './a/a1';\n@forward './a/a2';\n@forward './b/b';\n")
        .mock_file("multi_use/module/a/_variables.scss", "$a: default !default;\n")
        .mock_file("multi_use/module/a/a1.scss", "@forward './variables';\n@use './variables' as *;\n\n.a1 {\n  content: #{$a};\n}\n")
        .mock_file("multi_use/module/a/a2.scss", "@forward './variables';\n@use './variables' as *;\n\n.a2 {\n  content: #{$a};\n}\n")
        .mock_file("multi_use/module/b/_variables.scss", "$b: default !default;\n")
        .mock_file("multi_use/module/b/b.scss", "@forward './variables';\n@use './variables' as *;\n\n.b {\n  content: #{$b};\n}\n")
        .mock_file("single_use/module/_index.scss", "@forward './a/a';\n@forward './b/b';\n")
        .mock_file("single_use/module/a/_variables.scss", "$a: default !default;\n")
        .mock_file("single_use/module/a/a.scss", "@forward './variables';\n@use './variables' as *;\n\n.a {\n  content: #{$a};\n}\n")
        .mock_file("single_use/module/b/_variables.scss", "$b: default !default;\n")
        .mock_file("single_use/module/b/b.scss", "@forward './variables';\n@use './variables' as *;\n\n.b {\n  content: #{$b};\n}\n")
}

#[test]
#[ignore] // missing error
fn multi_use() {
    let runner = runner().with_cwd("multi_use");
    assert_eq!(
        runner.err(
            "@use \'module\' with (\
             \n  $a: \'a\',\
             \n  $b: \'b\',\
             \n  $missing: \'c\',\
             \n);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n4 |   $missing: \'c\',\
         \n  |   ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:3  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn single_use() {
    let runner = runner().with_cwd("single_use");
    assert_eq!(
        runner.err(
            "@use \'module\' with (\
             \n  $a: \'a\',\
             \n  $b: \'b\',\
             \n  $missing: \'c\',\
             \n);\n"
        ),
        "Error: This variable was not declared with !default in the @used module.\
         \n  ,\
         \n4 |   $missing: \'c\',\
         \n  |   ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:3  root stylesheet",
    );
}
