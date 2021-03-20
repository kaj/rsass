//! Tests auto-converted from "sass-spec/spec/non_conformant/sass_4_0/interpolation/function_name.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
