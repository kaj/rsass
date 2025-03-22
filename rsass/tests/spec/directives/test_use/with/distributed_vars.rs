//! Tests auto-converted from "sass-spec/spec/directives/use/with/distributed_vars.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("distributed_vars")
        .mock_file("repeated/module/_index.scss", "@forward './a/a1';\n@forward './a/a2';\n@forward './b/b';\n")
        .mock_file("repeated/module/a/_variables.scss", "$a: default !default;\n")
        .mock_file("repeated/module/a/a1.scss", "@forward './variables';\n@use './variables' as *;\n\n.a1 {\n  content: #{$a};\n}\n")
        .mock_file("repeated/module/a/a2.scss", "@forward './variables';\n@use './variables' as *;\n\n.a2 {\n  content: #{$a};\n}\n")
        .mock_file("repeated/module/b/_variables.scss", "$b: default !default;\n")
        .mock_file("repeated/module/b/b.scss", "@forward './variables';\n@use './variables' as *;\n\n.b {\n  content: #{$b};\n}\n")
        .mock_file("single_use/module/_index.scss", "@forward './a/a';\n@forward './b/b';\n")
        .mock_file("single_use/module/a/_variables.scss", "$a: default !default;\n")
        .mock_file("single_use/module/a/a.scss", "@forward './variables';\n@use './variables' as *;\n\n.a {\n  content: #{$a};\n}\n")
        .mock_file("single_use/module/b/_variables.scss", "$b: default !default;\n")
        .mock_file("single_use/module/b/b.scss", "@forward './variables';\n@use './variables' as *;\n\n.b {\n  content: #{$b};\n}\n")
}

#[test]
#[ignore] // wrong result
fn repeated() {
    let runner = runner().with_cwd("repeated");
    assert_eq!(
        runner.ok("@use \'module\' with (\
             \n  $a: \'a\',\
             \n  $b: \'b\',\
             \n);\n"),
        ".a1 {\
         \n  content: a;\
         \n}\
         \n.a2 {\
         \n  content: a;\
         \n}\
         \n.b {\
         \n  content: b;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn single_use() {
    let runner = runner().with_cwd("single_use");
    assert_eq!(
        runner.ok("@use \'module\' with (\
             \n  $a: \'a\',\
             \n  $b: \'b\',\
             \n);\n"),
        ".a {\
         \n  content: a;\
         \n}\
         \n.b {\
         \n  content: b;\
         \n}\n"
    );
}
