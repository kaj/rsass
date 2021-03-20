//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/scope.hrx"

#[test]
fn captures_inner_scope() {
    assert_eq!(
        crate::rsass(
            "@function add-two($v) {@error \"Should not be called\"}\
            \n.scope1 {\
            \n  @function add-two($v) {@error \"Should not be called\"}\
            \n  .scope2 {\
            \n    @function add-two($v) {@error \"Should not be called\"}\
            \n    .scope3 {\
            \n      @function add-two($v) {@return $v + 2}\
            \n\
            \n      // Like a normal function call, get-function() will always use the\
            \n      // innermost definition of a function.\
            \n      a: call(get-function(add-two), 10);\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".scope1 .scope2 .scope3 {\
        \n  a: 12;\
        \n}\
        \n"
    );
}
#[test]
fn stores_local_scope() {
    assert_eq!(
        crate::rsass(
            "$add-two-fn: null;\
            \n\
            \n.scope {\
            \n  @function add-two($v) {@return $v + 2}\
            \n\
            \n  // This function reference will still refer to this nested `add-two` function\
            \n  // even when it goes out of scope.\
            \n  $add-two-fn: get-function(add-two) !global;\
            \n}\
            \n\
            \na {b: call($add-two-fn, 10)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 12;\
        \n}\
        \n"
    );
}
