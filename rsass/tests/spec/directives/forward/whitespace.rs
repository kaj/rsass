//! Tests auto-converted from "sass-spec/spec/directives/forward/whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("whitespace")
        .mock_file("after_colon/sass/other.scss", "$a: 1 !default\n")
        .mock_file("after_colon/scss/other.scss", "$a: 1 !default\n")
        .mock_file("after_default/sass/other.scss", "$a: 1 !default\n")
        .mock_file("after_default/scss/other.scss", "$a: 1 !default\n")
        .mock_file("after_keyword/sass/other.scss", "")
        .mock_file("after_keyword/scss/other.scss", "")
        .mock_file("after_open_paren/scss/other.scss", "$a: 1 !default\n")
        .mock_file("after_paren/sass/other.scss", "$a: 1 !default\n")
        .mock_file(
            "after_variable_comma/sass/other.sass",
            "$a: 1 !default\n$c: 1 !default\n",
        )
        .mock_file(
            "after_variable_comma/scss/other.scss",
            "$a: 1 !default;\n$c: 1 !default\n",
        )
        .mock_file("before_close_paren/sass/other.scss", "$a: 1 !default\n")
        .mock_file("before_close_paren/scss/other.scss", "$a: 1 !default\n")
        .mock_file("before_colon/sass/other.scss", "$a: 1 !default\n")
        .mock_file("before_colon/scss/other.scss", "$a: 1 !default\n")
        .mock_file("before_default/sass/other.scss", "$a: 1 !default\n")
        .mock_file("before_default/scss/other.scss", "$a: 1 !default\n")
        .mock_file("before_keyword/scss/other.scss", "")
        .mock_file("before_url/sass/other.scss", "")
        .mock_file("before_url/scss/other.scss", "")
        .mock_file("error/before_keyword/sass/other.scss", "")
        .mock_file("hide/after_hide/sass/other.scss", "$a: 1 !default\n")
        .mock_file("show/after_a/sass/other.scss", "$a: 1 !default\n")
        .mock_file("show/after_comma/sass/other.scss", "$a: 1 !default\n")
        .mock_file("show/after_show/sass/other.scss", "$a: 1 !default\n")
}

mod after_colon {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("after_colon")
    }

    #[test]
    fn scss() {
        let runner = runner().with_cwd("scss");
        assert_eq!(
            runner.ok("@forward \"other\" with ($a:\
             \n  b)\n"),
            ""
        );
    }
}
mod after_default {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("after_default")
    }

    #[test]
    fn scss() {
        let runner = runner().with_cwd("scss");
        assert_eq!(
            runner.ok("@forward \"other\" with ($a: b !default\
             \n  )\n"),
            ""
        );
    }
}
mod after_keyword {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("after_keyword")
    }

    #[test]
    fn scss() {
        let runner = runner().with_cwd("scss");
        assert_eq!(
            runner.ok("@forward \"other\" as\
             \n  a-*\n"),
            ""
        );
    }
}
mod after_open_paren {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("after_open_paren")
    }

    #[test]
    fn scss() {
        let runner = runner().with_cwd("scss");
        assert_eq!(
            runner.ok("@forward \"other\" with (\
             \n  $a: b)\n"),
            ""
        );
    }
}
mod after_variable_comma {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("after_variable_comma")
    }

    #[test]
    fn scss() {
        let runner = runner().with_cwd("scss");
        assert_eq!(
            runner.ok("@forward \"other\" with ($a: b,\
             \n  $c: d)\n"),
            ""
        );
    }
}
mod before_close_paren {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("before_close_paren")
    }

    #[test]
    fn scss() {
        let runner = runner().with_cwd("scss");
        assert_eq!(
            runner.ok("@forward \"other\" with ($a: b\
             \n  )\n"),
            ""
        );
    }
}
mod before_colon {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("before_colon")
    }

    #[test]
    fn scss() {
        let runner = runner().with_cwd("scss");
        assert_eq!(
            runner.ok("@forward \"other\" with ($a\
             \n  : b)\n"),
            ""
        );
    }
}
mod before_default {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("before_default")
    }

    #[test]
    fn scss() {
        let runner = runner().with_cwd("scss");
        assert_eq!(
            runner.ok("@forward \"other\" with ($a: b\
             \n  !default)\n"),
            ""
        );
    }
}
mod before_keyword {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("before_keyword")
    }

    #[test]
    fn scss() {
        let runner = runner().with_cwd("scss");
        assert_eq!(
            runner.ok("@forward \"other\"\
             \n  as a-*\n"),
            ""
        );
    }
}
mod before_url {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("before_url")
    }

    #[test]
    fn scss() {
        let runner = runner().with_cwd("scss");
        assert_eq!(
            runner.ok("@forward\
             \n  \"other\"\n"),
            ""
        );
    }
}
