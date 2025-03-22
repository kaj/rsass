//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/53_escaped_quotes.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("53_escaped_quotes")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("[data-icon=\'test-1\']:before {\
             \n    content:\'\\\\\';\
             \n}\n\
             \n[data-icon=\'test-2\']:before {\
             \n    content:\'\\\'\';\
             \n}\n\
             \n[data-icon=\'test-3\']:before {\
             \n    content:\"\\\"\";\
             \n}\n\
             \n[data-icon=\'test-4\']:before {\
             \n    content:\'\\\\\';\
             \n}\n\
             \n[data-icon=\'test-5\']:before {\
             \n    content:\'\\\'\';\
             \n}\n\
             \n[data-icon=\'test-6\']:before {\
             \n    content:\"\\\"\";\
             \n}\n\
             \n$open-quote:    «;\
             \n$close-quote:   »;\n\
             \n$open-quote: \\201C;\
             \n$close-quote: \\201D;\n\
             \n.\\E9motion { \
             \nblah: hi; }\
             \n.\\E9 dition { \
             \nblah: hi; }\
             \n.\\0000E9dition { \
             \nblah: hi; }"),
        "@charset \"UTF-8\";\
         \n[data-icon=test-1]:before {\
         \n  content: \"\\\\\";\
         \n}\
         \n[data-icon=test-2]:before {\
         \n  content: \"\'\";\
         \n}\
         \n[data-icon=test-3]:before {\
         \n  content: \'\"\';\
         \n}\
         \n[data-icon=test-4]:before {\
         \n  content: \"\\\\\";\
         \n}\
         \n[data-icon=test-5]:before {\
         \n  content: \"\'\";\
         \n}\
         \n[data-icon=test-6]:before {\
         \n  content: \'\"\';\
         \n}\
         \n.émotion {\
         \n  blah: hi;\
         \n}\
         \n.édition {\
         \n  blah: hi;\
         \n}\
         \n.édition {\
         \n  blah: hi;\
         \n}\n"
    );
}
