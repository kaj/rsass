//! Tests auto-converted from "sass-spec/spec/css/plain/error/expression/if.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("if")
        .mock_file(
            "interp/and/plain.css",
            "a {b: if(#{css(1)} and css(2): c)}\n",
        )
        .mock_file("interp/direct/plain.css", "a {b: if(#{css()}: c)}\n")
        .mock_file("interp/not/plain.css", "a {b: if(not #{css()}: c)}\n")
        .mock_file(
            "interp/or/plain.css",
            "a {b: if(#{css(1)} or css(2): c)}\n",
        )
        .mock_file("interp/paren/plain.css", "a {b: if((#{css()}): c)}\n")
        .mock_file(
            "sass/and/plain.css",
            "a {b: if(sass(true) and css(): c)}\n",
        )
        .mock_file("sass/direct/plain.css", "a {b: if(sass(true): c)}\n")
        .mock_file("sass/not/plain.css", "a {b: if(not sass(true): c)}\n")
        .mock_file("sass/or/plain.css", "a {b: if(sass(true) or css(): c)}\n")
        .mock_file("sass/paren/plain.css", "a {b: if((sass(true)): c)}\n")
}

mod interp {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("interp")
    }

    #[test]
    #[ignore] // wrong error
    fn and() {
        let runner = runner().with_cwd("and");
        assert_eq!(
            runner.err("@use \"plain\";\n"),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | a {b: if(#{css(1)} and css(2): c)}\
         \n  |          ^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:10  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn direct() {
        let runner = runner().with_cwd("direct");
        assert_eq!(
            runner.err("@use \"plain\";\n"),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | a {b: if(#{css()}: c)}\
         \n  |          ^^^^^^^^\
         \n  \'\
         \n  plain.css 1:10  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn not() {
        let runner = runner().with_cwd("not");
        assert_eq!(
            runner.err("@use \"plain\";\n"),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | a {b: if(not #{css()}: c)}\
         \n  |              ^^^^^^^^\
         \n  \'\
         \n  plain.css 1:14  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn or() {
        let runner = runner().with_cwd("or");
        assert_eq!(
            runner.err("@use \"plain\";\n"),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | a {b: if(#{css(1)} or css(2): c)}\
         \n  |          ^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:10  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn paren() {
        let runner = runner().with_cwd("paren");
        assert_eq!(
            runner.err("@use \"plain\";\n"),
            "Error: Interpolation isn\'t allowed in plain CSS.\
         \n  ,\
         \n1 | a {b: if((#{css()}): c)}\
         \n  |           ^^^^^^^^\
         \n  \'\
         \n  plain.css 1:11  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
}
mod sass {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("sass")
    }

    #[test]
    #[ignore] // wrong error
    fn and() {
        let runner = runner().with_cwd("and");
        assert_eq!(
            runner.err("@use \"plain\";\n"),
            "Error: sass() conditions aren\'t allowed in plain CSS\
         \n  ,\
         \n1 | a {b: if(sass(true) and css(): c)}\
         \n  |          ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:10  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn direct() {
        let runner = runner().with_cwd("direct");
        assert_eq!(
            runner.err("@use \"plain\";\n"),
            "Error: sass() conditions aren\'t allowed in plain CSS\
         \n  ,\
         \n1 | a {b: if(sass(true): c)}\
         \n  |          ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:10  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn not() {
        let runner = runner().with_cwd("not");
        assert_eq!(
            runner.err("@use \"plain\";\n"),
            "Error: sass() conditions aren\'t allowed in plain CSS\
         \n  ,\
         \n1 | a {b: if(not sass(true): c)}\
         \n  |              ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:14  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn or() {
        let runner = runner().with_cwd("or");
        assert_eq!(
            runner.err("@use \"plain\";\n"),
            "Error: sass() conditions aren\'t allowed in plain CSS\
         \n  ,\
         \n1 | a {b: if(sass(true) or css(): c)}\
         \n  |          ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:10  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn paren() {
        let runner = runner().with_cwd("paren");
        assert_eq!(
            runner.err("@use \"plain\";\n"),
            "Error: sass() conditions aren\'t allowed in plain CSS\
         \n  ,\
         \n1 | a {b: if((sass(true)): c)}\
         \n  |           ^^^^^^^^^^\
         \n  \'\
         \n  plain.css 1:11  @use\
         \n  input.scss 1:1  root stylesheet",
        );
    }
}
