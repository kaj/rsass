//! Tests auto-converted from "sass-spec/spec/directives/use/extend/diamond.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("dependency/with_midstream_extend/_left.scss", "@use \"midstream\";\nin-left {\n  @extend in-midstream;\n  w: x;\n}\n")
        .mock_file("dependency/with_midstream_extend/_midstream.scss", "@use \"upstream\";\nin-midstream {@extend in-upstream}\n")
        .mock_file("dependency/with_midstream_extend/_right.scss", "@use \"midstream\";\nin-right {\n  @extend in-midstream;\n  y: z;\n}\n")
        .mock_file("dependency/with_midstream_extend/_upstream.scss", "in-upstream {a: b}\n")
        .mock_file("merge/_left.scss", "@use \"other\";\n\n.a {@extend %in-other}\n")
        .mock_file("merge/_other.scss", "%in-other.a {x: y}\n")
        .mock_file("merge/_right.scss", "@use \"other\";\n\n.b {@extend %in-other}\n")
}

mod dependency {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("dependency")
    }

    #[test]
    #[ignore] // wrong result
    fn with_midstream_extend() {
        let runner = runner().with_cwd("with_midstream_extend");
        assert_eq!(
            runner.ok("@use \"left\";\
             \n@use \"right\";\n"),
            "in-upstream, in-midstream, in-right, in-left {\
         \n  a: b;\
         \n}\
         \nin-left {\
         \n  w: x;\
         \n}\
         \nin-right {\
         \n  y: z;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn merge() {
    let runner = runner().with_cwd("merge");
    assert_eq!(
        runner.ok(
            "// Sibling modules can\'t extend one another\'s selectors, but they can be merged\
             \n// together into the same selector list if they extend the same thing. If they\
             \n// are, they should be optimized with respect to one another.\
             \n//\
             \n// In this case, _left.scss causes the selector \".a.a\" to be generated, which is\
             \n// simplified to \".a\". Then _right.scss causes \".a.b\" to be generated. \".a\" is a\
             \n// superselector of \".a.b\" and \".a\" has the same specificity as the extender,\
             \n// \".b\", so \".a.b\" can (and should) be optimized away.\
             \n@use \"left\";\
             \n@use \"right\";\n"
        ),
        ".a {\
         \n  x: y;\
         \n}\n"
    );
}
