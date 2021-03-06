//! Tests auto-converted from "sass-spec/spec/non_conformant/sass_4_0/interpolation"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/sass_4_0/interpolation/after_hyphen.hrx"
#[test]
fn after_hyphen() {
    assert_eq!(
        rsass(
            ".after-hyphen {\
            \n  // We want to treat interpolation as though it\'s a valid identifier character,\
            \n  // which means that if it comes after a hyphen or double-hyphen it should be\
            \n  // treated as part of an interpolated identifier that includes that hyphen.\
            \n  standalone-single: -#{foo};\
            \n  standalone-double: --#{foo};\
            \n\
            \n  // It also means that we shouldn\'t treat the hyphen as a subtraction\
            \n  // operation.\
            \n  adjacent-single: #{foo} -#{bar};\
            \n  adjacent-double: #{foo} --#{bar};\
            \n}\
            \n"
        )
        .unwrap(),
        ".after-hyphen {\
        \n  standalone-single: -foo;\
        \n  standalone-double: --foo;\
        \n  adjacent-single: foo -bar;\
        \n  adjacent-double: foo --bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/sass_4_0/interpolation/function_name.hrx"
#[test]
fn function_name() {
    assert_eq!(
        rsass(
            "@function identity($arg) {@return $arg}\
            \n\
            \n.function-name {\
            \n  start: #{1 + 1}foo(arg);\
            \n  mid: foo#{1 + 1}bar(arg);\
            \n  end: foo#{1 + 1}(arg);\
            \n  full: #{foo}(arg);\
            \n\
            \n  evaluates-args: foo#{1 + 1}bar(2 + 2);\
            \n  $list: 1, 2, 3, 4, 5;\
            \n  supports-splats: foo#{1 + 1}bar($list...);\
            \n\
            \n  not-built-in-function: qu#{o}te(arg);\
            \n  not-user-defined-function: id#{enti}ty(arg);\
            \n\
            \n  // In 3.5, this would interpret \"red()\" as a live function call.\
            \n  followed-by-function: b#{o}red(arg);\
            \n}"
        )
        .unwrap(),
        ".function-name {\
        \n  start: 2foo(arg);\
        \n  mid: foo2bar(arg);\
        \n  end: foo2(arg);\
        \n  full: foo(arg);\
        \n  evaluates-args: foo2bar(4);\
        \n  supports-splats: foo2bar(1, 2, 3, 4, 5);\
        \n  not-built-in-function: quote(arg);\
        \n  not-user-defined-function: identity(arg);\
        \n  followed-by-function: bored(arg);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/sass_4_0/interpolation/trailing_non_name_start.hrx"
#[test]
fn trailing_non_name_start() {
    assert_eq!(
        rsass(
            ".trailing-non-name-start {\
            \n  // We want to treat interpolation as though it\'s a valid identifier character,\
            \n  // which means that if it\'s followed by a [name character][] that\'s not a\
            \n  // [name start character][] it should still treat that as part of the\
            \n  // identifier.\
            \n  //\
            \n  // [name character]: https://drafts.csswg.org/css-syntax-3/#name-code-point\
            \n  // [name start character]: https://drafts.csswg.org/css-syntax-3/#name-start-code-point\
            \n  digit: foo#{bar}12;\
            \n  hyphen: foo#{bar}-12;\
            \n  double-hyphen: foo#{bar}--12;\
            \n}\
            \n"
        )
        .unwrap(),
        ".trailing-non-name-start {\
        \n  digit: foobar12;\
        \n  hyphen: foobar-12;\
        \n  double-hyphen: foobar--12;\
        \n}\
        \n"
    );
}
