extern crate rsass;
use rsass::{compile_scss, OutputStyle};

#[test]
fn t01_arg_eval() {
    check(
        "
        @function foo() {
          @return 1+2 3/4 5+6;
        }

        @mixin bar($x: 3/4) {
          bar-content: $x;
        }

        div {
          content: foobar(1+2 3/4 5+6, orange);
          content: append(1+2 2/3 5+6, orange);
          content: 1+2 2/3 5+6;
          content: type-of(2/3);
          content: type-of(orange);
          content: foo();
          @include bar();
        }
    ",
        "div {\n  \
         content: foobar(3 3/4 11, orange);\n  \
         content: 3 2/3 11 orange;\n  content: 3 2/3 11;\n  \
         content: number;\n  content: color;\n  content: 3 3/4 11;\n  \
         bar-content: 0.75;\n}\n",
    )
}

#[test]
fn features_units_level_3() {
    check(
        "foo {\n  foo: feature-exists('units-level-3');\n}\n",
        "foo {\n  foo: true;\n}\n",
    )
}

#[test]
fn keyframes() {
    check(
        "div {\n  color: #181818;\n}\n\n\
         @-webkit-keyframes uiDelayedFadeIn {\n\t0% { opacity: 0; }\n\t\
         50% { opacity: .5; }\n\t100% { opacity: 1; }\n}\n\n\
         @-webkit-keyframes bounce {\n\tfrom {\n\t\tleft: 0px;\n\t}\n\t\
         to {\n\t\tleft: 200px;\n\t}\n}\n\n\
         $name: bounce;\n\n\
         @-webkit-keyframes #{$name} {\n  blah: blee;\n}\n\n\
         @mixin fudge() {\n  @content;\n}\n\n\
         foo {\n  @include fudge() {\n    \
         div {\n      color: red;\n    }\n  }\n}\n\n\
         @-moz-document url-prefix() {\n\t.fl {\n\t\tfloat:left;\n\t\t\
         margin:12px 4px 0 0;\n\t\tpadding:0;\n\t\tfont-size:65px;\n\t\t\
         line-height:62%;\n\t\tcolor:#ba1820;\n\t}\n\t\
         .fs {\n\t\tfloat:left;\n\t\tmargin:12px 4px 10px 0;\n\t\t\
         padding:0; font-size:65px;\n\t\tline-height:62%;\n\t\t\
         color:#ba1820;\n\t}\n}",
        "div {\n  color: #181818;\n}\n\n\
         @-webkit-keyframes uiDelayedFadeIn {\n  \
         0% {\n    opacity: 0;\n  }\n  50% {\n    opacity: .5;\n  }\n  \
         100% {\n    opacity: 1;\n  }\n}\n\
         @-webkit-keyframes bounce {\n  \
         from {\n    left: 0px;\n  }\n  to {\n    left: 200px;\n  }\n}\n\
         @-webkit-keyframes bounce {\n  blah: blee;\n}\n\
         foo div {\n  color: red;\n}\n\n\
         @-moz-document url-prefix() {\n  .fl {\n    float: left;\n    \
         margin: 12px 4px 0 0;\n    padding: 0;\n    font-size: 65px;\n    \
         line-height: 62%;\n    color: #ba1820;\n  }\n  \
         .fs {\n    float: left;\n    margin: 12px 4px 10px 0;\n    \
         padding: 0;\n    font-size: 65px;\n    line-height: 62%;\n    \
         color: #ba1820;\n  }\n}\n",
    )
}

fn check(input: &str, expected: &str) {
    assert_eq!(
        compile_scss(input.as_ref(), OutputStyle::Expanded)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        expected
    );
}
