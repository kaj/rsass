//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/33_ambiguous_imports.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("33_ambiguous_imports")
        .mock_file("blir/_fudge.scss", "fudge {\n  color: brown;\n}")
        .mock_file("dir.scss", "@use \"sass:meta\";\ndir {\n  color: blue;\n}\n\n@include meta.load-css(\"blir/fudge\");")
        .mock_file("dir/whatever", "")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \nmain {\
             \n  color: red;\
             \n}\n\
             \n@include meta.load-css(\"dir\");"),
        "main {\
         \n  color: red;\
         \n}\
         \ndir {\
         \n  color: blue;\
         \n}\
         \nfudge {\
         \n  color: brown;\
         \n}\n"
    );
}
