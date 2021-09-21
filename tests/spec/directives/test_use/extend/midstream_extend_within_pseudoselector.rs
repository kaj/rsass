//! Tests auto-converted from "sass-spec/spec/directives/use/extend/midstream_extend_within_pseudoselector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("three_files/is/_midstream.scss", "@use \"upstream\";\n:is(in-midstream) {@extend in-upstream}\n")
        .mock_file("three_files/is/_upstream.scss", "in-upstream {a: b}\n")
        .mock_file("three_files/matches/_midstream.scss", "@use \"upstream\";\n:matches(in-midstream) {@extend in-upstream}\n")
        .mock_file("three_files/matches/_upstream.scss", "in-upstream {a: b}\n")
        .mock_file("two_files/is/_upstream.scss", "in-upstream {a: b}\n")
        .mock_file("two_files/matches/_upstream.scss", "in-upstream {a: b}\n")
}

mod three_files {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("three_files")
    }

    #[test]
    #[ignore] // wrong result
    fn is() {
        let runner = runner().with_cwd("is");
        assert_eq!(
            runner.ok("@use \"midstream\";\
             \nin-input {\
             \n  @extend in-midstream;\
             \n  y: z;\
             \n}\n"),
            "in-upstream, :is(in-midstream, in-input) {\
         \n  a: b;\
         \n}\
         \nin-input {\
         \n  y: z;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn matches() {
        let runner = runner().with_cwd("matches");
        assert_eq!(
            runner.ok("@use \"midstream\";\
             \nin-input {\
             \n  @extend in-midstream;\
             \n  y: z;\
             \n}\n"),
            "in-upstream, :matches(in-midstream, in-input) {\
         \n  a: b;\
         \n}\
         \nin-input {\
         \n  y: z;\
         \n}\n"
        );
    }
}
mod two_files {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("two_files")
    }

    #[test]
    #[ignore] // wrong result
    fn is() {
        let runner = runner().with_cwd("is");
        assert_eq!(
            runner.ok("@use \"upstream\";\
             \n:is(in-midstream) {@extend in-upstream}\n\
             \nin-input {\
             \n  @extend in-midstream;\
             \n  y: z;\
             \n}\n"),
            "in-upstream, :is(in-midstream, in-input) {\
         \n  a: b;\
         \n}\
         \nin-input {\
         \n  y: z;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn matches() {
        let runner = runner().with_cwd("matches");
        assert_eq!(
            runner.ok("@use \"upstream\";\
             \n:matches(in-midstream) {@extend in-upstream}\n\
             \nin-input {\
             \n  @extend in-midstream;\
             \n  y: z;\
             \n}\n"),
            "in-upstream, :matches(in-midstream, in-input) {\
         \n  a: b;\
         \n}\
         \nin-input {\
         \n  y: z;\
         \n}\n"
        );
    }
}
