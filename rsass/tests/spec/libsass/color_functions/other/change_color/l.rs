//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/other/change-color/l.hrx"

#[allow(unused)]
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
         \n  c1: rgb(5.1, 0, 0);\
         \n  c2: rgb(10.2, 0, 0);\
         \n  c3: rgb(15.3, 0, 0);\
         \n  c4: rgb(20.4, 0, 0);\
         \n  c5: rgb(25.5, 0, 0);\
         \n  c6: rgb(30.6, 0, 0);\
         \n  c7: rgb(35.7, 0, 0);\
         \n  c8: rgb(40.8, 0, 0);\
         \n  c9: rgb(45.9, 0, 0);\
         \n  c10: #330000;\
         \n  c11: rgb(56.1, 0, 0);\
         \n  c12: rgb(61.2, 0, 0);\
         \n  c13: rgb(66.3, 0, 0);\
         \n  c14: rgb(71.4, 0, 0);\
         \n  c15: rgb(76.5, 0, 0);\
         \n  c16: rgb(81.6, 0, 0);\
         \n  c17: rgb(86.7, 0, 0);\
         \n  c18: rgb(91.8, 0, 0);\
         \n  c19: rgb(96.9, 0, 0);\
         \n  c20: #660000;\
         \n  c21: rgb(107.1, 0, 0);\
         \n  c22: rgb(112.2, 0, 0);\
         \n  c23: rgb(117.3, 0, 0);\
         \n  c24: rgb(122.4, 0, 0);\
         \n  c25: rgb(127.5, 0, 0);\
         \n  c26: rgb(132.6, 0, 0);\
         \n  c27: rgb(137.7, 0, 0);\
         \n  c28: rgb(142.8, 0, 0);\
         \n  c29: rgb(147.9, 0, 0);\
         \n  c30: #990000;\
         \n  c31: rgb(158.1, 0, 0);\
         \n  c32: rgb(163.2, 0, 0);\
         \n  c33: rgb(168.3, 0, 0);\
         \n  c34: rgb(173.4, 0, 0);\
         \n  c35: rgb(178.5, 0, 0);\
         \n  c36: rgb(183.6, 0, 0);\
         \n  c37: rgb(188.7, 0, 0);\
         \n  c38: rgb(193.8, 0, 0);\
         \n  c39: rgb(198.9, 0, 0);\
         \n  c40: #cc0000;\
         \n  c41: rgb(209.1, 0, 0);\
         \n  c42: rgb(214.2, 0, 0);\
         \n  c43: rgb(219.3, 0, 0);\
         \n  c44: rgb(224.4, 0, 0);\
         \n  c45: rgb(229.5, 0, 0);\
         \n  c46: rgb(234.6, 0, 0);\
         \n  c47: rgb(239.7, 0, 0);\
         \n  c48: rgb(244.8, 0, 0);\
         \n  c49: rgb(249.9, 0, 0);\
         \n  c50: red;\
         \n  c51: rgb(255, 5.1, 5.1);\
         \n  c52: rgb(255, 10.2, 10.2);\
         \n  c53: rgb(255, 15.3, 15.3);\
         \n  c54: rgb(255, 20.4, 20.4);\
         \n  c55: rgb(255, 25.5, 25.5);\
         \n  c56: rgb(255, 30.6, 30.6);\
         \n  c57: rgb(255, 35.7, 35.7);\
         \n  c58: rgb(255, 40.8, 40.8);\
         \n  c59: rgb(255, 45.9, 45.9);\
         \n  c60: #ff3333;\
         \n  c61: rgb(255, 56.1, 56.1);\
         \n  c62: rgb(255, 61.2, 61.2);\
         \n  c63: rgb(255, 66.3, 66.3);\
         \n  c64: rgb(255, 71.4, 71.4);\
         \n  c65: rgb(255, 76.5, 76.5);\
         \n  c66: rgb(255, 81.6, 81.6);\
         \n  c67: rgb(255, 86.7, 86.7);\
         \n  c68: rgb(255, 91.8, 91.8);\
         \n  c69: rgb(255, 96.9, 96.9);\
         \n  c70: #ff6666;\
         \n  c71: rgb(255, 107.1, 107.1);\
         \n  c72: rgb(255, 112.2, 112.2);\
         \n  c73: rgb(255, 117.3, 117.3);\
         \n  c74: rgb(255, 122.4, 122.4);\
         \n  c75: rgb(255, 127.5, 127.5);\
         \n  c76: rgb(255, 132.6, 132.6);\
         \n  c77: rgb(255, 137.7, 137.7);\
         \n  c78: rgb(255, 142.8, 142.8);\
         \n  c79: rgb(255, 147.9, 147.9);\
         \n  c80: #ff9999;\
         \n  c81: rgb(255, 158.1, 158.1);\
         \n  c82: rgb(255, 163.2, 163.2);\
         \n  c83: rgb(255, 168.3, 168.3);\
         \n  c84: rgb(255, 173.4, 173.4);\
         \n  c85: rgb(255, 178.5, 178.5);\
         \n  c86: rgb(255, 183.6, 183.6);\
         \n  c87: rgb(255, 188.7, 188.7);\
         \n  c88: rgb(255, 193.8, 193.8);\
         \n  c89: rgb(255, 198.9, 198.9);\
         \n  c90: #ffcccc;\
         \n  c91: rgb(255, 209.1, 209.1);\
         \n  c92: rgb(255, 214.2, 214.2);\
         \n  c93: rgb(255, 219.3, 219.3);\
         \n  c94: rgb(255, 224.4, 224.4);\
         \n  c95: rgb(255, 229.5, 229.5);\
         \n  c96: rgb(255, 234.6, 234.6);\
         \n  c97: rgb(255, 239.7, 239.7);\
         \n  c98: rgb(255, 244.8, 244.8);\
         \n  c99: rgb(255, 249.9, 249.9);\
         \n}\n"
    );
}
