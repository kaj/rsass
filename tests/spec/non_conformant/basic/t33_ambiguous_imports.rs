//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/33_ambiguous_imports.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("blir/_fudge.scss", "fudge {\n  color: brown;\n}")
        .mock_file(
            "dir.scss",
            "dir {\n  color: blue;\n}\n\n@import \"blir/fudge\";",
        )
        .mock_file("dir/whatever", "")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("main {\
             \n  color: red;\
             \n}\n\
             \n@import \"dir\";"),
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
