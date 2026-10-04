//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/other/change-color/l.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("l")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \nfoo {\
             \n  // c-1: change-color(red,$lightness:-1%);\
             \n  c0: color.change(red,$lightness:0%);\
             \n  c1: color.change(red,$lightness:1%);\
             \n  c2: color.change(red,$lightness:2%);\
             \n  c3: color.change(red,$lightness:3%);\
             \n  c4: color.change(red,$lightness:4%);\
             \n  c5: color.change(red,$lightness:5%);\
             \n  c6: color.change(red,$lightness:6%);\
             \n  c7: color.change(red,$lightness:7%);\
             \n  c8: color.change(red,$lightness:8%);\
             \n  c9: color.change(red,$lightness:9%);\
             \n  c10: color.change(red,$lightness:10%);\
             \n  c11: color.change(red,$lightness:11%);\
             \n  c12: color.change(red,$lightness:12%);\
             \n  c13: color.change(red,$lightness:13%);\
             \n  c14: color.change(red,$lightness:14%);\
             \n  c15: color.change(red,$lightness:15%);\
             \n  c16: color.change(red,$lightness:16%);\
             \n  c17: color.change(red,$lightness:17%);\
             \n  c18: color.change(red,$lightness:18%);\
             \n  c19: color.change(red,$lightness:19%);\
             \n  c20: color.change(red,$lightness:20%);\
             \n  c21: color.change(red,$lightness:21%);\
             \n  c22: color.change(red,$lightness:22%);\
             \n  c23: color.change(red,$lightness:23%);\
             \n  c24: color.change(red,$lightness:24%);\
             \n  c25: color.change(red,$lightness:25%);\
             \n  c26: color.change(red,$lightness:26%);\
             \n  c27: color.change(red,$lightness:27%);\
             \n  c28: color.change(red,$lightness:28%);\
             \n  c29: color.change(red,$lightness:29%);\
             \n  c30: color.change(red,$lightness:30%);\
             \n  c31: color.change(red,$lightness:31%);\
             \n  c32: color.change(red,$lightness:32%);\
             \n  c33: color.change(red,$lightness:33%);\
             \n  c34: color.change(red,$lightness:34%);\
             \n  c35: color.change(red,$lightness:35%);\
             \n  c36: color.change(red,$lightness:36%);\
             \n  c37: color.change(red,$lightness:37%);\
             \n  c38: color.change(red,$lightness:38%);\
             \n  c39: color.change(red,$lightness:39%);\
             \n  c40: color.change(red,$lightness:40%);\
             \n  c41: color.change(red,$lightness:41%);\
             \n  c42: color.change(red,$lightness:42%);\
             \n  c43: color.change(red,$lightness:43%);\
             \n  c44: color.change(red,$lightness:44%);\
             \n  c45: color.change(red,$lightness:45%);\
             \n  c46: color.change(red,$lightness:46%);\
             \n  c47: color.change(red,$lightness:47%);\
             \n  c48: color.change(red,$lightness:48%);\
             \n  c49: color.change(red,$lightness:49%);\
             \n  c50: color.change(red,$lightness:50%);\
             \n  c51: color.change(red,$lightness:51%);\
             \n  c52: color.change(red,$lightness:52%);\
             \n  c53: color.change(red,$lightness:53%);\
             \n  c54: color.change(red,$lightness:54%);\
             \n  c55: color.change(red,$lightness:55%);\
             \n  c56: color.change(red,$lightness:56%);\
             \n  c57: color.change(red,$lightness:57%);\
             \n  c58: color.change(red,$lightness:58%);\
             \n  c59: color.change(red,$lightness:59%);\
             \n  c60: color.change(red,$lightness:60%);\
             \n  c61: color.change(red,$lightness:61%);\
             \n  c62: color.change(red,$lightness:62%);\
             \n  c63: color.change(red,$lightness:63%);\
             \n  c64: color.change(red,$lightness:64%);\
             \n  c65: color.change(red,$lightness:65%);\
             \n  c66: color.change(red,$lightness:66%);\
             \n  c67: color.change(red,$lightness:67%);\
             \n  c68: color.change(red,$lightness:68%);\
             \n  c69: color.change(red,$lightness:69%);\
             \n  c70: color.change(red,$lightness:70%);\
             \n  c71: color.change(red,$lightness:71%);\
             \n  c72: color.change(red,$lightness:72%);\
             \n  c73: color.change(red,$lightness:73%);\
             \n  c74: color.change(red,$lightness:74%);\
             \n  c75: color.change(red,$lightness:75%);\
             \n  c76: color.change(red,$lightness:76%);\
             \n  c77: color.change(red,$lightness:77%);\
             \n  c78: color.change(red,$lightness:78%);\
             \n  c79: color.change(red,$lightness:79%);\
             \n  c80: color.change(red,$lightness:80%);\
             \n  c81: color.change(red,$lightness:81%);\
             \n  c82: color.change(red,$lightness:82%);\
             \n  c83: color.change(red,$lightness:83%);\
             \n  c84: color.change(red,$lightness:84%);\
             \n  c85: color.change(red,$lightness:85%);\
             \n  c86: color.change(red,$lightness:86%);\
             \n  c87: color.change(red,$lightness:87%);\
             \n  c88: color.change(red,$lightness:88%);\
             \n  c89: color.change(red,$lightness:89%);\
             \n  c90: color.change(red,$lightness:90%);\
             \n  c91: color.change(red,$lightness:91%);\
             \n  c92: color.change(red,$lightness:92%);\
             \n  c93: color.change(red,$lightness:93%);\
             \n  c94: color.change(red,$lightness:94%);\
             \n  c95: color.change(red,$lightness:95%);\
             \n  c96: color.change(red,$lightness:96%);\
             \n  c97: color.change(red,$lightness:97%);\
             \n  c98: color.change(red,$lightness:98%);\
             \n  c99: color.change(red,$lightness:99%);\
             \n  // c100: change-color(red,$lightness:100%);\
             \n}\n"),
        "foo {\
         \n  c0: black;\
         \n  c1: rgb(2%, 0%, 0%);\
         \n  c2: rgb(4%, 0%, 0%);\
         \n  c3: rgb(6%, 0%, 0%);\
         \n  c4: rgb(8%, 0%, 0%);\
         \n  c5: rgb(10%, 0%, 0%);\
         \n  c6: rgb(12%, 0%, 0%);\
         \n  c7: rgb(14%, 0%, 0%);\
         \n  c8: rgb(16%, 0%, 0%);\
         \n  c9: rgb(18%, 0%, 0%);\
         \n  c10: #330000;\
         \n  c11: rgb(22%, 0%, 0%);\
         \n  c12: rgb(24%, 0%, 0%);\
         \n  c13: rgb(26%, 0%, 0%);\
         \n  c14: rgb(28%, 0%, 0%);\
         \n  c15: rgb(30%, 0%, 0%);\
         \n  c16: rgb(32%, 0%, 0%);\
         \n  c17: rgb(34%, 0%, 0%);\
         \n  c18: rgb(36%, 0%, 0%);\
         \n  c19: rgb(38%, 0%, 0%);\
         \n  c20: #660000;\
         \n  c21: rgb(42%, 0%, 0%);\
         \n  c22: rgb(44%, 0%, 0%);\
         \n  c23: rgb(46%, 0%, 0%);\
         \n  c24: rgb(48%, 0%, 0%);\
         \n  c25: rgb(50%, 0%, 0%);\
         \n  c26: rgb(52%, 0%, 0%);\
         \n  c27: rgb(54%, 0%, 0%);\
         \n  c28: rgb(56%, 0%, 0%);\
         \n  c29: rgb(58%, 0%, 0%);\
         \n  c30: #990000;\
         \n  c31: rgb(62%, 0%, 0%);\
         \n  c32: rgb(64%, 0%, 0%);\
         \n  c33: rgb(66%, 0%, 0%);\
         \n  c34: rgb(68%, 0%, 0%);\
         \n  c35: rgb(70%, 0%, 0%);\
         \n  c36: rgb(72%, 0%, 0%);\
         \n  c37: rgb(74%, 0%, 0%);\
         \n  c38: rgb(76%, 0%, 0%);\
         \n  c39: rgb(78%, 0%, 0%);\
         \n  c40: #cc0000;\
         \n  c41: rgb(82%, 0%, 0%);\
         \n  c42: rgb(84%, 0%, 0%);\
         \n  c43: rgb(86%, 0%, 0%);\
         \n  c44: rgb(88%, 0%, 0%);\
         \n  c45: rgb(90%, 0%, 0%);\
         \n  c46: rgb(92%, 0%, 0%);\
         \n  c47: rgb(94%, 0%, 0%);\
         \n  c48: rgb(96%, 0%, 0%);\
         \n  c49: rgb(98%, 0%, 0%);\
         \n  c50: red;\
         \n  c51: rgb(100%, 2%, 2%);\
         \n  c52: rgb(100%, 4%, 4%);\
         \n  c53: rgb(100%, 6%, 6%);\
         \n  c54: rgb(100%, 8%, 8%);\
         \n  c55: rgb(100%, 10%, 10%);\
         \n  c56: rgb(100%, 12%, 12%);\
         \n  c57: rgb(100%, 14%, 14%);\
         \n  c58: rgb(100%, 16%, 16%);\
         \n  c59: rgb(100%, 18%, 18%);\
         \n  c60: #ff3333;\
         \n  c61: rgb(100%, 22%, 22%);\
         \n  c62: rgb(100%, 24%, 24%);\
         \n  c63: rgb(100%, 26%, 26%);\
         \n  c64: rgb(100%, 28%, 28%);\
         \n  c65: rgb(100%, 30%, 30%);\
         \n  c66: rgb(100%, 32%, 32%);\
         \n  c67: rgb(100%, 34%, 34%);\
         \n  c68: rgb(100%, 36%, 36%);\
         \n  c69: rgb(100%, 38%, 38%);\
         \n  c70: #ff6666;\
         \n  c71: rgb(100%, 42%, 42%);\
         \n  c72: rgb(100%, 44%, 44%);\
         \n  c73: rgb(100%, 46%, 46%);\
         \n  c74: rgb(100%, 48%, 48%);\
         \n  c75: rgb(100%, 50%, 50%);\
         \n  c76: rgb(100%, 52%, 52%);\
         \n  c77: rgb(100%, 54%, 54%);\
         \n  c78: rgb(100%, 56%, 56%);\
         \n  c79: rgb(100%, 58%, 58%);\
         \n  c80: #ff9999;\
         \n  c81: rgb(100%, 62%, 62%);\
         \n  c82: rgb(100%, 64%, 64%);\
         \n  c83: rgb(100%, 66%, 66%);\
         \n  c84: rgb(100%, 68%, 68%);\
         \n  c85: rgb(100%, 70%, 70%);\
         \n  c86: rgb(100%, 72%, 72%);\
         \n  c87: rgb(100%, 74%, 74%);\
         \n  c88: rgb(100%, 76%, 76%);\
         \n  c89: rgb(100%, 78%, 78%);\
         \n  c90: #ffcccc;\
         \n  c91: rgb(100%, 82%, 82%);\
         \n  c92: rgb(100%, 84%, 84%);\
         \n  c93: rgb(100%, 86%, 86%);\
         \n  c94: rgb(100%, 88%, 88%);\
         \n  c95: rgb(100%, 90%, 90%);\
         \n  c96: rgb(100%, 92%, 92%);\
         \n  c97: rgb(100%, 94%, 94%);\
         \n  c98: rgb(100%, 96%, 96%);\
         \n  c99: rgb(100%, 98%, 98%);\
         \n}\n"
    );
}
