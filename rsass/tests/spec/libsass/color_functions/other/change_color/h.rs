//! Tests auto-converted from "sass-spec/spec/libsass/color-functions/other/change-color/h.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("h")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \nfoo {\
             \n  c-1: color.change(red,$hue:-1);\
             \n  c0: color.change(red,$hue:0);\
             \n  c1: color.change(red,$hue:1);\
             \n  c2: color.change(red,$hue:2);\
             \n  c3: color.change(red,$hue:3);\
             \n  c4: color.change(red,$hue:4);\
             \n  c5: color.change(red,$hue:5);\
             \n  c6: color.change(red,$hue:6);\
             \n  c7: color.change(red,$hue:7);\
             \n  c8: color.change(red,$hue:8);\
             \n  c9: color.change(red,$hue:9);\
             \n  c10: color.change(red,$hue:10);\
             \n  c11: color.change(red,$hue:11);\
             \n  c12: color.change(red,$hue:12);\
             \n  c13: color.change(red,$hue:13);\
             \n  c14: color.change(red,$hue:14);\
             \n  c15: color.change(red,$hue:15);\
             \n  c16: color.change(red,$hue:16);\
             \n  c17: color.change(red,$hue:17);\
             \n  c18: color.change(red,$hue:18);\
             \n  c19: color.change(red,$hue:19);\
             \n  c20: color.change(red,$hue:20);\
             \n  c21: color.change(red,$hue:21);\
             \n  c22: color.change(red,$hue:22);\
             \n  c23: color.change(red,$hue:23);\
             \n  c24: color.change(red,$hue:24);\
             \n  c25: color.change(red,$hue:25);\
             \n  c26: color.change(red,$hue:26);\
             \n  c27: color.change(red,$hue:27);\
             \n  c28: color.change(red,$hue:28);\
             \n  c29: color.change(red,$hue:29);\
             \n  c30: color.change(red,$hue:30);\
             \n  c31: color.change(red,$hue:31);\
             \n  c32: color.change(red,$hue:32);\
             \n  c33: color.change(red,$hue:33);\
             \n  c34: color.change(red,$hue:34);\
             \n  c35: color.change(red,$hue:35);\
             \n  c36: color.change(red,$hue:36);\
             \n  c37: color.change(red,$hue:37);\
             \n  c38: color.change(red,$hue:38);\
             \n  c39: color.change(red,$hue:39);\
             \n  c40: color.change(red,$hue:40);\
             \n  c41: color.change(red,$hue:41);\
             \n  c42: color.change(red,$hue:42);\
             \n  c43: color.change(red,$hue:43);\
             \n  c44: color.change(red,$hue:44);\
             \n  c45: color.change(red,$hue:45);\
             \n  c46: color.change(red,$hue:46);\
             \n  c47: color.change(red,$hue:47);\
             \n  c48: color.change(red,$hue:48);\
             \n  c49: color.change(red,$hue:49);\
             \n  c50: color.change(red,$hue:50);\
             \n  c51: color.change(red,$hue:51);\
             \n  c52: color.change(red,$hue:52);\
             \n  c53: color.change(red,$hue:53);\
             \n  c54: color.change(red,$hue:54);\
             \n  c55: color.change(red,$hue:55);\
             \n  c56: color.change(red,$hue:56);\
             \n  c57: color.change(red,$hue:57);\
             \n  c58: color.change(red,$hue:58);\
             \n  c59: color.change(red,$hue:59);\
             \n  c60: color.change(red,$hue:60);\
             \n  c61: color.change(red,$hue:61);\
             \n  c62: color.change(red,$hue:62);\
             \n  c63: color.change(red,$hue:63);\
             \n  c64: color.change(red,$hue:64);\
             \n  c65: color.change(red,$hue:65);\
             \n  c66: color.change(red,$hue:66);\
             \n  c67: color.change(red,$hue:67);\
             \n  c68: color.change(red,$hue:68);\
             \n  c69: color.change(red,$hue:69);\
             \n  c70: color.change(red,$hue:70);\
             \n  c71: color.change(red,$hue:71);\
             \n  c72: color.change(red,$hue:72);\
             \n  c73: color.change(red,$hue:73);\
             \n  c74: color.change(red,$hue:74);\
             \n  c75: color.change(red,$hue:75);\
             \n  c76: color.change(red,$hue:76);\
             \n  c77: color.change(red,$hue:77);\
             \n  c78: color.change(red,$hue:78);\
             \n  c79: color.change(red,$hue:79);\
             \n  c80: color.change(red,$hue:80);\
             \n  c81: color.change(red,$hue:81);\
             \n  c82: color.change(red,$hue:82);\
             \n  c83: color.change(red,$hue:83);\
             \n  c84: color.change(red,$hue:84);\
             \n  c85: color.change(red,$hue:85);\
             \n  c86: color.change(red,$hue:86);\
             \n  c87: color.change(red,$hue:87);\
             \n  c88: color.change(red,$hue:88);\
             \n  c89: color.change(red,$hue:89);\
             \n  c90: color.change(red,$hue:90);\
             \n  c91: color.change(red,$hue:91);\
             \n  c92: color.change(red,$hue:92);\
             \n  c93: color.change(red,$hue:93);\
             \n  c94: color.change(red,$hue:94);\
             \n  c95: color.change(red,$hue:95);\
             \n  c96: color.change(red,$hue:96);\
             \n  c97: color.change(red,$hue:97);\
             \n  c98: color.change(red,$hue:98);\
             \n  c99: color.change(red,$hue:99);\
             \n  c100: color.change(red,$hue:100);\
             \n  c101: color.change(red,$hue:101);\
             \n  c102: color.change(red,$hue:102);\
             \n  c103: color.change(red,$hue:103);\
             \n  c104: color.change(red,$hue:104);\
             \n  c105: color.change(red,$hue:105);\
             \n  c106: color.change(red,$hue:106);\
             \n  c107: color.change(red,$hue:107);\
             \n  c108: color.change(red,$hue:108);\
             \n  c109: color.change(red,$hue:109);\
             \n  c110: color.change(red,$hue:110);\
             \n  c111: color.change(red,$hue:111);\
             \n  c112: color.change(red,$hue:112);\
             \n  c113: color.change(red,$hue:113);\
             \n  c114: color.change(red,$hue:114);\
             \n  c115: color.change(red,$hue:115);\
             \n  c116: color.change(red,$hue:116);\
             \n  c117: color.change(red,$hue:117);\
             \n  c118: color.change(red,$hue:118);\
             \n  c119: color.change(red,$hue:119);\
             \n  c120: color.change(red,$hue:120);\
             \n  c121: color.change(red,$hue:121);\
             \n  c122: color.change(red,$hue:122);\
             \n  c123: color.change(red,$hue:123);\
             \n  c124: color.change(red,$hue:124);\
             \n  c125: color.change(red,$hue:125);\
             \n  c126: color.change(red,$hue:126);\
             \n  c127: color.change(red,$hue:127);\
             \n  c128: color.change(red,$hue:128);\
             \n  c129: color.change(red,$hue:129);\
             \n  c130: color.change(red,$hue:130);\
             \n  c131: color.change(red,$hue:131);\
             \n  c132: color.change(red,$hue:132);\
             \n  c133: color.change(red,$hue:133);\
             \n  c134: color.change(red,$hue:134);\
             \n  c135: color.change(red,$hue:135);\
             \n  c136: color.change(red,$hue:136);\
             \n  c137: color.change(red,$hue:137);\
             \n  c138: color.change(red,$hue:138);\
             \n  c139: color.change(red,$hue:139);\
             \n  c140: color.change(red,$hue:140);\
             \n  c141: color.change(red,$hue:141);\
             \n  c142: color.change(red,$hue:142);\
             \n  c143: color.change(red,$hue:143);\
             \n  c144: color.change(red,$hue:144);\
             \n  c145: color.change(red,$hue:145);\
             \n  c146: color.change(red,$hue:146);\
             \n  c147: color.change(red,$hue:147);\
             \n  c148: color.change(red,$hue:148);\
             \n  c149: color.change(red,$hue:149);\
             \n  c150: color.change(red,$hue:150);\
             \n  c151: color.change(red,$hue:151);\
             \n  c152: color.change(red,$hue:152);\
             \n  c153: color.change(red,$hue:153);\
             \n  c154: color.change(red,$hue:154);\
             \n  c155: color.change(red,$hue:155);\
             \n  c156: color.change(red,$hue:156);\
             \n  c157: color.change(red,$hue:157);\
             \n  c158: color.change(red,$hue:158);\
             \n  c159: color.change(red,$hue:159);\
             \n  c160: color.change(red,$hue:160);\
             \n  c161: color.change(red,$hue:161);\
             \n  c162: color.change(red,$hue:162);\
             \n  c163: color.change(red,$hue:163);\
             \n  c164: color.change(red,$hue:164);\
             \n  c165: color.change(red,$hue:165);\
             \n  c166: color.change(red,$hue:166);\
             \n  c167: color.change(red,$hue:167);\
             \n  c168: color.change(red,$hue:168);\
             \n  c169: color.change(red,$hue:169);\
             \n  c170: color.change(red,$hue:170);\
             \n  c171: color.change(red,$hue:171);\
             \n  c172: color.change(red,$hue:172);\
             \n  c173: color.change(red,$hue:173);\
             \n  c174: color.change(red,$hue:174);\
             \n  c175: color.change(red,$hue:175);\
             \n  c176: color.change(red,$hue:176);\
             \n  c177: color.change(red,$hue:177);\
             \n  c178: color.change(red,$hue:178);\
             \n  c179: color.change(red,$hue:179);\
             \n  c180: color.change(red,$hue:180);\
             \n  c181: color.change(red,$hue:181);\
             \n  c182: color.change(red,$hue:182);\
             \n  c183: color.change(red,$hue:183);\
             \n  c184: color.change(red,$hue:184);\
             \n  c185: color.change(red,$hue:185);\
             \n  c186: color.change(red,$hue:186);\
             \n  c187: color.change(red,$hue:187);\
             \n  c188: color.change(red,$hue:188);\
             \n  c189: color.change(red,$hue:189);\
             \n  c190: color.change(red,$hue:190);\
             \n  c191: color.change(red,$hue:191);\
             \n  c192: color.change(red,$hue:192);\
             \n  c193: color.change(red,$hue:193);\
             \n  c194: color.change(red,$hue:194);\
             \n  c195: color.change(red,$hue:195);\
             \n  c196: color.change(red,$hue:196);\
             \n  c197: color.change(red,$hue:197);\
             \n  c198: color.change(red,$hue:198);\
             \n  c199: color.change(red,$hue:199);\
             \n  c200: color.change(red,$hue:200);\
             \n  c201: color.change(red,$hue:201);\
             \n  c202: color.change(red,$hue:202);\
             \n  c203: color.change(red,$hue:203);\
             \n  c204: color.change(red,$hue:204);\
             \n  c205: color.change(red,$hue:205);\
             \n  c206: color.change(red,$hue:206);\
             \n  c207: color.change(red,$hue:207);\
             \n  c208: color.change(red,$hue:208);\
             \n  c209: color.change(red,$hue:209);\
             \n  c210: color.change(red,$hue:210);\
             \n  c211: color.change(red,$hue:211);\
             \n  c212: color.change(red,$hue:212);\
             \n  c213: color.change(red,$hue:213);\
             \n  c214: color.change(red,$hue:214);\
             \n  c215: color.change(red,$hue:215);\
             \n  c216: color.change(red,$hue:216);\
             \n  c217: color.change(red,$hue:217);\
             \n  c218: color.change(red,$hue:218);\
             \n  c219: color.change(red,$hue:219);\
             \n  c220: color.change(red,$hue:220);\
             \n  c221: color.change(red,$hue:221);\
             \n  c222: color.change(red,$hue:222);\
             \n  c223: color.change(red,$hue:223);\
             \n  c224: color.change(red,$hue:224);\
             \n  c225: color.change(red,$hue:225);\
             \n  c226: color.change(red,$hue:226);\
             \n  c227: color.change(red,$hue:227);\
             \n  c228: color.change(red,$hue:228);\
             \n  c229: color.change(red,$hue:229);\
             \n  c230: color.change(red,$hue:230);\
             \n  c231: color.change(red,$hue:231);\
             \n  c232: color.change(red,$hue:232);\
             \n  c233: color.change(red,$hue:233);\
             \n  c234: color.change(red,$hue:234);\
             \n  c235: color.change(red,$hue:235);\
             \n  c236: color.change(red,$hue:236);\
             \n  c237: color.change(red,$hue:237);\
             \n  c238: color.change(red,$hue:238);\
             \n  c239: color.change(red,$hue:239);\
             \n  c240: color.change(red,$hue:240);\
             \n  c241: color.change(red,$hue:241);\
             \n  c242: color.change(red,$hue:242);\
             \n  c243: color.change(red,$hue:243);\
             \n  c244: color.change(red,$hue:244);\
             \n  c245: color.change(red,$hue:245);\
             \n  c246: color.change(red,$hue:246);\
             \n  c247: color.change(red,$hue:247);\
             \n  c248: color.change(red,$hue:248);\
             \n  c249: color.change(red,$hue:249);\
             \n  c250: color.change(red,$hue:250);\
             \n  c251: color.change(red,$hue:251);\
             \n  c252: color.change(red,$hue:252);\
             \n  c253: color.change(red,$hue:253);\
             \n  c254: color.change(red,$hue:254);\
             \n  c255: color.change(red,$hue:255);\
             \n  c256: color.change(red,$hue:256);\
             \n  c257: color.change(red,$hue:257);\
             \n  c258: color.change(red,$hue:258);\
             \n  c259: color.change(red,$hue:259);\
             \n  c260: color.change(red,$hue:260);\
             \n  c261: color.change(red,$hue:261);\
             \n  c262: color.change(red,$hue:262);\
             \n  c263: color.change(red,$hue:263);\
             \n  c264: color.change(red,$hue:264);\
             \n  c265: color.change(red,$hue:265);\
             \n  c266: color.change(red,$hue:266);\
             \n  c267: color.change(red,$hue:267);\
             \n  c268: color.change(red,$hue:268);\
             \n  c269: color.change(red,$hue:269);\
             \n  c270: color.change(red,$hue:270);\
             \n  c271: color.change(red,$hue:271);\
             \n  c272: color.change(red,$hue:272);\
             \n  c273: color.change(red,$hue:273);\
             \n  c274: color.change(red,$hue:274);\
             \n  c275: color.change(red,$hue:275);\
             \n  c276: color.change(red,$hue:276);\
             \n  c277: color.change(red,$hue:277);\
             \n  c278: color.change(red,$hue:278);\
             \n  c279: color.change(red,$hue:279);\
             \n  c280: color.change(red,$hue:280);\
             \n  c281: color.change(red,$hue:281);\
             \n  c282: color.change(red,$hue:282);\
             \n  c283: color.change(red,$hue:283);\
             \n  c284: color.change(red,$hue:284);\
             \n  c285: color.change(red,$hue:285);\
             \n  c286: color.change(red,$hue:286);\
             \n  c287: color.change(red,$hue:287);\
             \n  c288: color.change(red,$hue:288);\
             \n  c289: color.change(red,$hue:289);\
             \n  c290: color.change(red,$hue:290);\
             \n  c291: color.change(red,$hue:291);\
             \n  c292: color.change(red,$hue:292);\
             \n  c293: color.change(red,$hue:293);\
             \n  c294: color.change(red,$hue:294);\
             \n  c295: color.change(red,$hue:295);\
             \n  c296: color.change(red,$hue:296);\
             \n  c297: color.change(red,$hue:297);\
             \n  c298: color.change(red,$hue:298);\
             \n  c299: color.change(red,$hue:299);\
             \n  c300: color.change(red,$hue:300);\
             \n  c301: color.change(red,$hue:301);\
             \n  c302: color.change(red,$hue:302);\
             \n  c303: color.change(red,$hue:303);\
             \n  c304: color.change(red,$hue:304);\
             \n  c305: color.change(red,$hue:305);\
             \n  c306: color.change(red,$hue:306);\
             \n  c307: color.change(red,$hue:307);\
             \n  c308: color.change(red,$hue:308);\
             \n  c309: color.change(red,$hue:309);\
             \n  c310: color.change(red,$hue:310);\
             \n  c311: color.change(red,$hue:311);\
             \n  c312: color.change(red,$hue:312);\
             \n  c313: color.change(red,$hue:313);\
             \n  c314: color.change(red,$hue:314);\
             \n  c315: color.change(red,$hue:315);\
             \n  c316: color.change(red,$hue:316);\
             \n  c317: color.change(red,$hue:317);\
             \n  c318: color.change(red,$hue:318);\
             \n  c319: color.change(red,$hue:319);\
             \n  c320: color.change(red,$hue:320);\
             \n  c321: color.change(red,$hue:321);\
             \n  c322: color.change(red,$hue:322);\
             \n  c323: color.change(red,$hue:323);\
             \n  c324: color.change(red,$hue:324);\
             \n  c325: color.change(red,$hue:325);\
             \n  c326: color.change(red,$hue:326);\
             \n  c327: color.change(red,$hue:327);\
             \n  c328: color.change(red,$hue:328);\
             \n  c329: color.change(red,$hue:329);\
             \n  c330: color.change(red,$hue:330);\
             \n  c331: color.change(red,$hue:331);\
             \n  c332: color.change(red,$hue:332);\
             \n  c333: color.change(red,$hue:333);\
             \n  c334: color.change(red,$hue:334);\
             \n  c335: color.change(red,$hue:335);\
             \n  c336: color.change(red,$hue:336);\
             \n  c337: color.change(red,$hue:337);\
             \n  c338: color.change(red,$hue:338);\
             \n  c339: color.change(red,$hue:339);\
             \n  c340: color.change(red,$hue:340);\
             \n  c341: color.change(red,$hue:341);\
             \n  c342: color.change(red,$hue:342);\
             \n  c343: color.change(red,$hue:343);\
             \n  c344: color.change(red,$hue:344);\
             \n  c345: color.change(red,$hue:345);\
             \n  c346: color.change(red,$hue:346);\
             \n  c347: color.change(red,$hue:347);\
             \n  c348: color.change(red,$hue:348);\
             \n  c349: color.change(red,$hue:349);\
             \n  c350: color.change(red,$hue:350);\
             \n  c351: color.change(red,$hue:351);\
             \n  c352: color.change(red,$hue:352);\
             \n  c353: color.change(red,$hue:353);\
             \n  c354: color.change(red,$hue:354);\
             \n  c355: color.change(red,$hue:355);\
             \n  c356: color.change(red,$hue:356);\
             \n  c357: color.change(red,$hue:357);\
             \n  c358: color.change(red,$hue:358);\
             \n  c359: color.change(red,$hue:359);\
             \n  c360: color.change(red,$hue:360);\
             \n}\n"),
        "foo {\
         \n  c-1: rgb(255, 0, 4.25);\
         \n  c0: red;\
         \n  c1: rgb(255, 4.25, 0);\
         \n  c2: rgb(255, 8.5, 0);\
         \n  c3: rgb(255, 12.75, 0);\
         \n  c4: #ff1100;\
         \n  c5: rgb(255, 21.25, 0);\
         \n  c6: rgb(255, 25.5, 0);\
         \n  c7: rgb(255, 29.75, 0);\
         \n  c8: #ff2200;\
         \n  c9: rgb(255, 38.25, 0);\
         \n  c10: rgb(255, 42.5, 0);\
         \n  c11: rgb(255, 46.75, 0);\
         \n  c12: #ff3300;\
         \n  c13: rgb(255, 55.25, 0);\
         \n  c14: rgb(255, 59.5, 0);\
         \n  c15: rgb(255, 63.75, 0);\
         \n  c16: #ff4400;\
         \n  c17: rgb(255, 72.25, 0);\
         \n  c18: rgb(255, 76.5, 0);\
         \n  c19: rgb(255, 80.75, 0);\
         \n  c20: #ff5500;\
         \n  c21: rgb(255, 89.25, 0);\
         \n  c22: rgb(255, 93.5, 0);\
         \n  c23: rgb(255, 97.75, 0);\
         \n  c24: #ff6600;\
         \n  c25: rgb(255, 106.25, 0);\
         \n  c26: rgb(255, 110.5, 0);\
         \n  c27: rgb(255, 114.75, 0);\
         \n  c28: #ff7700;\
         \n  c29: rgb(255, 123.25, 0);\
         \n  c30: rgb(255, 127.5, 0);\
         \n  c31: rgb(255, 131.75, 0);\
         \n  c32: #ff8800;\
         \n  c33: rgb(255, 140.25, 0);\
         \n  c34: rgb(255, 144.5, 0);\
         \n  c35: rgb(255, 148.75, 0);\
         \n  c36: #ff9900;\
         \n  c37: rgb(255, 157.25, 0);\
         \n  c38: rgb(255, 161.5, 0);\
         \n  c39: rgb(255, 165.75, 0);\
         \n  c40: #ffaa00;\
         \n  c41: rgb(255, 174.25, 0);\
         \n  c42: rgb(255, 178.5, 0);\
         \n  c43: rgb(255, 182.75, 0);\
         \n  c44: #ffbb00;\
         \n  c45: rgb(255, 191.25, 0);\
         \n  c46: rgb(255, 195.5, 0);\
         \n  c47: rgb(255, 199.75, 0);\
         \n  c48: #ffcc00;\
         \n  c49: rgb(255, 208.25, 0);\
         \n  c50: rgb(255, 212.5, 0);\
         \n  c51: rgb(255, 216.75, 0);\
         \n  c52: #ffdd00;\
         \n  c53: rgb(255, 225.25, 0);\
         \n  c54: rgb(255, 229.5, 0);\
         \n  c55: rgb(255, 233.75, 0);\
         \n  c56: #ffee00;\
         \n  c57: rgb(255, 242.25, 0);\
         \n  c58: rgb(255, 246.5, 0);\
         \n  c59: rgb(255, 250.75, 0);\
         \n  c60: yellow;\
         \n  c61: rgb(250.75, 255, 0);\
         \n  c62: rgb(246.5, 255, 0);\
         \n  c63: rgb(242.25, 255, 0);\
         \n  c64: #eeff00;\
         \n  c65: rgb(233.75, 255, 0);\
         \n  c66: rgb(229.5, 255, 0);\
         \n  c67: rgb(225.25, 255, 0);\
         \n  c68: #ddff00;\
         \n  c69: rgb(216.75, 255, 0);\
         \n  c70: rgb(212.5, 255, 0);\
         \n  c71: rgb(208.25, 255, 0);\
         \n  c72: #ccff00;\
         \n  c73: rgb(199.75, 255, 0);\
         \n  c74: rgb(195.5, 255, 0);\
         \n  c75: rgb(191.25, 255, 0);\
         \n  c76: #bbff00;\
         \n  c77: rgb(182.75, 255, 0);\
         \n  c78: rgb(178.5, 255, 0);\
         \n  c79: rgb(174.25, 255, 0);\
         \n  c80: #aaff00;\
         \n  c81: rgb(165.75, 255, 0);\
         \n  c82: rgb(161.5, 255, 0);\
         \n  c83: rgb(157.25, 255, 0);\
         \n  c84: #99ff00;\
         \n  c85: rgb(148.75, 255, 0);\
         \n  c86: rgb(144.5, 255, 0);\
         \n  c87: rgb(140.25, 255, 0);\
         \n  c88: #88ff00;\
         \n  c89: rgb(131.75, 255, 0);\
         \n  c90: rgb(127.5, 255, 0);\
         \n  c91: rgb(123.25, 255, 0);\
         \n  c92: #77ff00;\
         \n  c93: rgb(114.75, 255, 0);\
         \n  c94: rgb(110.5, 255, 0);\
         \n  c95: rgb(106.25, 255, 0);\
         \n  c96: #66ff00;\
         \n  c97: rgb(97.75, 255, 0);\
         \n  c98: rgb(93.5, 255, 0);\
         \n  c99: rgb(89.25, 255, 0);\
         \n  c100: #55ff00;\
         \n  c101: rgb(80.75, 255, 0);\
         \n  c102: rgb(76.5, 255, 0);\
         \n  c103: rgb(72.25, 255, 0);\
         \n  c104: #44ff00;\
         \n  c105: rgb(63.75, 255, 0);\
         \n  c106: rgb(59.5, 255, 0);\
         \n  c107: rgb(55.25, 255, 0);\
         \n  c108: #33ff00;\
         \n  c109: rgb(46.75, 255, 0);\
         \n  c110: rgb(42.5, 255, 0);\
         \n  c111: rgb(38.25, 255, 0);\
         \n  c112: #22ff00;\
         \n  c113: rgb(29.75, 255, 0);\
         \n  c114: rgb(25.5, 255, 0);\
         \n  c115: rgb(21.25, 255, 0);\
         \n  c116: #11ff00;\
         \n  c117: rgb(12.75, 255, 0);\
         \n  c118: rgb(8.5, 255, 0);\
         \n  c119: rgb(4.25, 255, 0);\
         \n  c120: lime;\
         \n  c121: rgb(0, 255, 4.25);\
         \n  c122: rgb(0, 255, 8.5);\
         \n  c123: rgb(0, 255, 12.75);\
         \n  c124: #00ff11;\
         \n  c125: rgb(0, 255, 21.25);\
         \n  c126: rgb(0, 255, 25.5);\
         \n  c127: rgb(0, 255, 29.75);\
         \n  c128: #00ff22;\
         \n  c129: rgb(0, 255, 38.25);\
         \n  c130: rgb(0, 255, 42.5);\
         \n  c131: rgb(0, 255, 46.75);\
         \n  c132: #00ff33;\
         \n  c133: rgb(0, 255, 55.25);\
         \n  c134: rgb(0, 255, 59.5);\
         \n  c135: rgb(0, 255, 63.75);\
         \n  c136: #00ff44;\
         \n  c137: rgb(0, 255, 72.25);\
         \n  c138: rgb(0, 255, 76.5);\
         \n  c139: rgb(0, 255, 80.75);\
         \n  c140: #00ff55;\
         \n  c141: rgb(0, 255, 89.25);\
         \n  c142: rgb(0, 255, 93.5);\
         \n  c143: rgb(0, 255, 97.75);\
         \n  c144: #00ff66;\
         \n  c145: rgb(0, 255, 106.25);\
         \n  c146: rgb(0, 255, 110.5);\
         \n  c147: rgb(0, 255, 114.75);\
         \n  c148: #00ff77;\
         \n  c149: rgb(0, 255, 123.25);\
         \n  c150: rgb(0, 255, 127.5);\
         \n  c151: rgb(0, 255, 131.75);\
         \n  c152: #00ff88;\
         \n  c153: rgb(0, 255, 140.25);\
         \n  c154: rgb(0, 255, 144.5);\
         \n  c155: rgb(0, 255, 148.75);\
         \n  c156: #00ff99;\
         \n  c157: rgb(0, 255, 157.25);\
         \n  c158: rgb(0, 255, 161.5);\
         \n  c159: rgb(0, 255, 165.75);\
         \n  c160: #00ffaa;\
         \n  c161: rgb(0, 255, 174.25);\
         \n  c162: rgb(0, 255, 178.5);\
         \n  c163: rgb(0, 255, 182.75);\
         \n  c164: #00ffbb;\
         \n  c165: rgb(0, 255, 191.25);\
         \n  c166: rgb(0, 255, 195.5);\
         \n  c167: rgb(0, 255, 199.75);\
         \n  c168: #00ffcc;\
         \n  c169: rgb(0, 255, 208.25);\
         \n  c170: rgb(0, 255, 212.5);\
         \n  c171: rgb(0, 255, 216.75);\
         \n  c172: #00ffdd;\
         \n  c173: rgb(0, 255, 225.25);\
         \n  c174: rgb(0, 255, 229.5);\
         \n  c175: rgb(0, 255, 233.75);\
         \n  c176: #00ffee;\
         \n  c177: rgb(0, 255, 242.25);\
         \n  c178: rgb(0, 255, 246.5);\
         \n  c179: rgb(0, 255, 250.75);\
         \n  c180: aqua;\
         \n  c181: rgb(0, 250.75, 255);\
         \n  c182: rgb(0, 246.5, 255);\
         \n  c183: rgb(0, 242.25, 255);\
         \n  c184: #00eeff;\
         \n  c185: rgb(0, 233.75, 255);\
         \n  c186: rgb(0, 229.5, 255);\
         \n  c187: rgb(0, 225.25, 255);\
         \n  c188: #00ddff;\
         \n  c189: rgb(0, 216.75, 255);\
         \n  c190: rgb(0, 212.5, 255);\
         \n  c191: rgb(0, 208.25, 255);\
         \n  c192: #00ccff;\
         \n  c193: rgb(0, 199.75, 255);\
         \n  c194: rgb(0, 195.5, 255);\
         \n  c195: rgb(0, 191.25, 255);\
         \n  c196: #00bbff;\
         \n  c197: rgb(0, 182.75, 255);\
         \n  c198: rgb(0, 178.5, 255);\
         \n  c199: rgb(0, 174.25, 255);\
         \n  c200: #00aaff;\
         \n  c201: rgb(0, 165.75, 255);\
         \n  c202: rgb(0, 161.5, 255);\
         \n  c203: rgb(0, 157.25, 255);\
         \n  c204: #0099ff;\
         \n  c205: rgb(0, 148.75, 255);\
         \n  c206: rgb(0, 144.5, 255);\
         \n  c207: rgb(0, 140.25, 255);\
         \n  c208: #0088ff;\
         \n  c209: rgb(0, 131.75, 255);\
         \n  c210: rgb(0, 127.5, 255);\
         \n  c211: rgb(0, 123.25, 255);\
         \n  c212: #0077ff;\
         \n  c213: rgb(0, 114.75, 255);\
         \n  c214: rgb(0, 110.5, 255);\
         \n  c215: rgb(0, 106.25, 255);\
         \n  c216: #0066ff;\
         \n  c217: rgb(0, 97.75, 255);\
         \n  c218: rgb(0, 93.5, 255);\
         \n  c219: rgb(0, 89.25, 255);\
         \n  c220: #0055ff;\
         \n  c221: rgb(0, 80.75, 255);\
         \n  c222: rgb(0, 76.5, 255);\
         \n  c223: rgb(0, 72.25, 255);\
         \n  c224: #0044ff;\
         \n  c225: rgb(0, 63.75, 255);\
         \n  c226: rgb(0, 59.5, 255);\
         \n  c227: rgb(0, 55.25, 255);\
         \n  c228: #0033ff;\
         \n  c229: rgb(0, 46.75, 255);\
         \n  c230: rgb(0, 42.5, 255);\
         \n  c231: rgb(0, 38.25, 255);\
         \n  c232: #0022ff;\
         \n  c233: rgb(0, 29.75, 255);\
         \n  c234: rgb(0, 25.5, 255);\
         \n  c235: rgb(0, 21.25, 255);\
         \n  c236: #0011ff;\
         \n  c237: rgb(0, 12.75, 255);\
         \n  c238: rgb(0, 8.5, 255);\
         \n  c239: rgb(0, 4.25, 255);\
         \n  c240: blue;\
         \n  c241: rgb(4.25, 0, 255);\
         \n  c242: rgb(8.5, 0, 255);\
         \n  c243: rgb(12.75, 0, 255);\
         \n  c244: #1100ff;\
         \n  c245: rgb(21.25, 0, 255);\
         \n  c246: rgb(25.5, 0, 255);\
         \n  c247: rgb(29.75, 0, 255);\
         \n  c248: #2200ff;\
         \n  c249: rgb(38.25, 0, 255);\
         \n  c250: rgb(42.5, 0, 255);\
         \n  c251: rgb(46.75, 0, 255);\
         \n  c252: #3300ff;\
         \n  c253: rgb(55.25, 0, 255);\
         \n  c254: rgb(59.5, 0, 255);\
         \n  c255: rgb(63.75, 0, 255);\
         \n  c256: #4400ff;\
         \n  c257: rgb(72.25, 0, 255);\
         \n  c258: rgb(76.5, 0, 255);\
         \n  c259: rgb(80.75, 0, 255);\
         \n  c260: #5500ff;\
         \n  c261: rgb(89.25, 0, 255);\
         \n  c262: rgb(93.5, 0, 255);\
         \n  c263: rgb(97.75, 0, 255);\
         \n  c264: #6600ff;\
         \n  c265: rgb(106.25, 0, 255);\
         \n  c266: rgb(110.5, 0, 255);\
         \n  c267: rgb(114.75, 0, 255);\
         \n  c268: #7700ff;\
         \n  c269: rgb(123.25, 0, 255);\
         \n  c270: rgb(127.5, 0, 255);\
         \n  c271: rgb(131.75, 0, 255);\
         \n  c272: #8800ff;\
         \n  c273: rgb(140.25, 0, 255);\
         \n  c274: rgb(144.5, 0, 255);\
         \n  c275: rgb(148.75, 0, 255);\
         \n  c276: #9900ff;\
         \n  c277: rgb(157.25, 0, 255);\
         \n  c278: rgb(161.5, 0, 255);\
         \n  c279: rgb(165.75, 0, 255);\
         \n  c280: #aa00ff;\
         \n  c281: rgb(174.25, 0, 255);\
         \n  c282: rgb(178.5, 0, 255);\
         \n  c283: rgb(182.75, 0, 255);\
         \n  c284: #bb00ff;\
         \n  c285: rgb(191.25, 0, 255);\
         \n  c286: rgb(195.5, 0, 255);\
         \n  c287: rgb(199.75, 0, 255);\
         \n  c288: #cc00ff;\
         \n  c289: rgb(208.25, 0, 255);\
         \n  c290: rgb(212.5, 0, 255);\
         \n  c291: rgb(216.75, 0, 255);\
         \n  c292: #dd00ff;\
         \n  c293: rgb(225.25, 0, 255);\
         \n  c294: rgb(229.5, 0, 255);\
         \n  c295: rgb(233.75, 0, 255);\
         \n  c296: #ee00ff;\
         \n  c297: rgb(242.25, 0, 255);\
         \n  c298: rgb(246.5, 0, 255);\
         \n  c299: rgb(250.75, 0, 255);\
         \n  c300: fuchsia;\
         \n  c301: rgb(255, 0, 250.75);\
         \n  c302: rgb(255, 0, 246.5);\
         \n  c303: rgb(255, 0, 242.25);\
         \n  c304: #ff00ee;\
         \n  c305: rgb(255, 0, 233.75);\
         \n  c306: rgb(255, 0, 229.5);\
         \n  c307: rgb(255, 0, 225.25);\
         \n  c308: #ff00dd;\
         \n  c309: rgb(255, 0, 216.75);\
         \n  c310: rgb(255, 0, 212.5);\
         \n  c311: rgb(255, 0, 208.25);\
         \n  c312: #ff00cc;\
         \n  c313: rgb(255, 0, 199.75);\
         \n  c314: rgb(255, 0, 195.5);\
         \n  c315: rgb(255, 0, 191.25);\
         \n  c316: #ff00bb;\
         \n  c317: rgb(255, 0, 182.75);\
         \n  c318: rgb(255, 0, 178.5);\
         \n  c319: rgb(255, 0, 174.25);\
         \n  c320: #ff00aa;\
         \n  c321: rgb(255, 0, 165.75);\
         \n  c322: rgb(255, 0, 161.5);\
         \n  c323: rgb(255, 0, 157.25);\
         \n  c324: #ff0099;\
         \n  c325: rgb(255, 0, 148.75);\
         \n  c326: rgb(255, 0, 144.5);\
         \n  c327: rgb(255, 0, 140.25);\
         \n  c328: #ff0088;\
         \n  c329: rgb(255, 0, 131.75);\
         \n  c330: rgb(255, 0, 127.5);\
         \n  c331: rgb(255, 0, 123.25);\
         \n  c332: #ff0077;\
         \n  c333: rgb(255, 0, 114.75);\
         \n  c334: rgb(255, 0, 110.5);\
         \n  c335: rgb(255, 0, 106.25);\
         \n  c336: #ff0066;\
         \n  c337: rgb(255, 0, 97.75);\
         \n  c338: rgb(255, 0, 93.5);\
         \n  c339: rgb(255, 0, 89.25);\
         \n  c340: #ff0055;\
         \n  c341: rgb(255, 0, 80.75);\
         \n  c342: rgb(255, 0, 76.5);\
         \n  c343: rgb(255, 0, 72.25);\
         \n  c344: #ff0044;\
         \n  c345: rgb(255, 0, 63.75);\
         \n  c346: rgb(255, 0, 59.5);\
         \n  c347: rgb(255, 0, 55.25);\
         \n  c348: #ff0033;\
         \n  c349: rgb(255, 0, 46.75);\
         \n  c350: rgb(255, 0, 42.5);\
         \n  c351: rgb(255, 0, 38.25);\
         \n  c352: #ff0022;\
         \n  c353: rgb(255, 0, 29.75);\
         \n  c354: rgb(255, 0, 25.5);\
         \n  c355: rgb(255, 0, 21.25);\
         \n  c356: #ff0011;\
         \n  c357: rgb(255, 0, 12.75);\
         \n  c358: rgb(255, 0, 8.5);\
         \n  c359: rgb(255, 0, 4.25);\
         \n  c360: red;\
         \n}\n"
    );
}
