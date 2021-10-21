use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! colors {
    ($($x:expr => $r:expr, $g:expr, $b:expr);+) => {
        {
            let mut output = Vec::new();
            $(
                output.push(Colour::new(String::from($x), String::from(""), $r, $g, $b));
            )+
            output
        }
    };
    ($($x:expr, $y:expr => $r:expr, $g:expr, $b:expr);+) => {
        {
            let mut output = Vec::new();
            $(
                output.push(Colour::new(String::from($x), String::from($y), $r, $g, $b));
            )+
            output
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.0, self.1, self.2)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Colour {
    pub name: String,
    pub alias: String,
    pub rgb: Vec<u8>,
    pub hex: String,
    pub hex2: String,
}

impl Colour {
    pub fn new(name: String, alias: String, r: u8, g: u8, b: u8) -> Self {
        Self {
            name,
            alias,
            rgb: vec![r, g, b],
            hex: format!("#{:02x}{:02x}{:02x}", r, g, b),
            hex2: format!("{:02x}{:02x}{:02x}", r, g, b),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tradition {
    pub colors: Vec<Colour>,
}

impl Tradition {
    pub fn new() -> Self {
        Self {
            colors: colors![
                "人籁", "renlai" => 158, 188, 25;
                "青粲", "qincan" => 195, 217, 78;
                "翠缥", "cuipiao" => 183, 211, 50;
                "水龙吟", "shuilongyin" => 132, 167, 41;
                "碧山", "bishan" => 119, 150, 73;
                "石发", "shifa" => 106, 141, 82;
                "菉竹", "luzhu" => 105, 142, 106;
                "庭芜绿", "tingwulv" => 104, 148, 92;
                "葱倩", "congqing" => 108, 134, 80;
                "漆姑", "qigu" => 93, 131, 81;
                "翠微", "cuiwei" => 76, 128, 69;
                "芰荷", "zhihe" => 79, 121, 74;
                "青青", "qinqin" => 79, 111, 70;
                "翠虬", "cuiqiu" => 68, 106, 55;
                "官绿", "guanlv" => 42, 110, 63;
                "油绿", "youlv" => 93, 114, 89;
                "莓莓", "meimei" => 78, 101, 72;
                "螺青", "luoqin" => 63, 80, 59;
                "春辰", "chunchen" => 169, 190, 123;
                "麴尘", "quchen" => 192, 208, 157;
                "欧碧", "oubi" => 192, 214, 149;
                "苍葭", "cangjia" => 168, 191, 143;
                "兰苕", "lanzhao" => 168, 183, 140;
                "青玉案", "qingyuan" => 168, 176, 146;
                "碧滋", "bizi" => 144, 160, 125;
                "瓷秘", "cimi" => 191, 192, 150;
                "筠雾", "junwu" => 213, 209, 174;
                "行香子", "xingxiangzi" => 191, 185, 156;
                "鸣珂", "mingke" => 179, 181, 156;
                "琬琰", "wanyan" => 169, 168, 134;
                "出岫", "chuyou" => 169, 167, 115;
                "王刍", "wangzou" => 169, 159, 112;
                "春碧", "chunbi" => 157, 157, 130;
                "执大象", "zhidaxiang" => 145, 145, 119;
                "青圭", "qinggui" => 146, 144, 93;
                "绿沈", "chunsheng" => 147, 143, 76;
                "风入松", "fengrusong" => 134, 140, 78;
                "荩箧", "jingxia" => 135, 125, 82;
                "绞衣", "jiaoyi" => 127, 117, 76;
                "素綦", "suxi" => 89, 83, 51;
                "苍筤", "cangliang" => 153, 188, 172;
                "天缥", "tianpiao" => 213, 235, 225;
                "卵色", "luanse" => 213, 227, 212;
                "沧浪", "canglang" => 177, 213, 200;
                "葭菼", "jiayan" => 202, 215, 197;
                "山岚", "shanlan" => 190, 210, 187;
                "冰台", "bintai" => 190, 202, 183;
                "青古", "qinggu" => 178, 189, 169;
                "醽醁", "linglu" => 166, 186, 177; // ling lu
                "渌波", "lubo" => 155, 180, 150;
                "青楸", "qingqiu" => 129, 163, 128;
                "缥碧", "piaobi" => 128, 164, 146;
                "翠涛", "cuitao" => 129, 157, 142;
                "青梅", "qingmei" => 119, 138, 119;
                "雀梅", "quemei" => 119, 138, 111;
                "苔古", "taigu" => 121, 131, 108;
                "蕉月",  "jiaoyue" => 134, 144, 138;
                "千山翠", "qianshancui" => 107, 125, 115;
                "翕赩", "xixi" => 95, 118, 106;
                "结绿", "jiejv" => 85, 95, 77;
                "绿云", "lvyun" => 69, 73, 61;
                // 深红
                "丹罽", "danji" => 230, 0, 18;
                "黄丹", "huangdan" => 234, 85, 20;
                "檎丹", "qindan" => 233, 72, 41;
                "银朱", "yinzhu" => 209, 41, 32;
                "洛神珠", "luoshenzhu" => 210, 57, 24;
                "珊瑚赫", "shanhuhe" => 193, 44, 31;
                "朱孔阳", "zhukongyang" => 184, 26, 53;
                "丹雘", "danhuo" => 200, 22, 29;
                "水华朱", "shuihuazhu" => 167, 33, 38;
                "胭脂虫", "yanzhichong" => 171, 29, 34;
                "朱樱", "zhuying" => 143, 29, 34;
                "大繎", "daran" => 130, 35, 39;
                "顺圣", "shunsheng" => 124, 25, 30;
                "爵头", "juetou" => 99, 18, 22;
                "麒麟竭", "qilinjie" => 76, 30, 26;
                "苕荣", "tiaorong" => 237, 109, 61;
                "扶光", "fuguang" => 240, 194, 162;
                "十样锦", "shiyangjin" => 248, 198, 181;
                "海天霞", "haitianxia" => 243, 166, 148;
                "骍刚", "xinggang" => 245, 176, 135;
                "朱颜酡", "zhuyantuo"=> 242, 154, 118;
                "赪霞", "chengxia" => 241, 143, 96;
                "赪尾", "chengwei" => 239, 132, 93;
                "缙云", "jinyun" => 238, 121, 89;
                "小红", "xiaohong" => 230, 119, 90;
                "琼琚", "qiongju" => 215, 127, 102;
                "岱赭", "daizhe" => 221, 107, 79;
                "朱柿", "zhushi" => 237, 109, 70;
                "赩炽", "xichi" => 203, 82,62;
                "鹤顶红", "hedinghong" => 210, 71, 53;
                "纁黄", "xunhuang" => 186, 81, 64;
                "棠梨", "tangli" => 177, 90, 67;
                "朱殷", "zhuyin" => 185, 58, 38;
                "石榴裙", "shiliuqun" => 177, 59, 46;
                "朱草", "zhucao" => 166, 84, 54;
                "赤灵", "chiling" => 149, 64, 36;
                "佛赤", "fochi" => 143, 61, 44;
                "綪筏", "qingfa" => 158, 42, 34;
                "朱湛", "zhuzhan" => 149, 48, 46;
                "丹秫", "danshu" => 135, 52, 36;
                "木兰", "mulan" => 102, 43, 31;
                // 粉红
                "杨妃", "yangfei" => 240, 145, 160;
                "盈盈", "yingying" => 249, 211, 227;
                "银红", "yinhong" => 231, 202, 211;
                "粉米", "fenmi" => 239, 196, 206;
                "桃夭", "taoyao" => 246, 190, 200;
                "水红", "shuihong" => 236, 176, 193;
                "夕岚", "xilan" => 227, 173, 185;
                "彤管", "tongguan" => 226, 162, 172;
                "咸池", "xianchi" => 218, 169, 169;
                "莲红", "lianhong" => 217, 160, 179;
                "雌霓", "cini" => 207, 146, 158;
                "縓缘", "quanyuan" => 206, 136, 146;
                "长春", "changchun" => 220 ,107, 130;
                "渥赭", "wozhe" => 221, 107, 123;
                "红䵂","hongzhi" => 205, 115,114;
                "紫梅", "zimei" => 187, 122, 140;
                "绛纱", "jiangsha" => 178, 119, 119;
                "如藘", "rulu" => 163, 95, 101;
                "美人祭", "meirenji" => 195, 92, 106;
                "唇脂", "chunzhi" => 194, 81, 96;
                "鞓红", "tinghong" => 176, 69, 82;
                "葡萄褐", "putaohe" => 158, 105, 109;
                "蚩尤旗", "chiyouqi" => 168, 88, 88;
                "紫矿", "zikuang" => 158, 78, 86;
                "紫诰", "zigao" => 118, 85, 93;
                "苏方", "sufang" => 129, 71, 76;
                "霁红", "jihong" => 124, 68, 73;
                "蜜褐", "mihe" => 104, 54, 50;
                "福色", "fuse" => 102, 43, 47;
                // 紫色
                "黪紫", "canzi" => 204, 115, 160;
                "龙膏烛", "longgaozhu" => 222, 130, 167;
                "苏梅", "sumei" => 221, 118, 148;
                "琅玕紫", "langanzi" => 203, 92, 131;
                "胭脂水", "yanzhishui" => 185, 90, 137;
                "紫茎屏风", "zijingpingfeng" => 167, 98, 131;
                "红踯躅", "hongzhizhu" => 184, 53, 112;
                "胭脂紫", "yanzhizi" => 176, 67, 111;
                "魏红", "weihong" => 167, 55, 102;
                "紫府", "zifu" => 153, 93, 127;
                "魏紫", "weizi" => 144, 55, 84;
                "地血", "dixue" => 129, 70, 98;
                "芥拾紫", "jieshizi" => 96, 38, 65;
                "油紫", "youzi" => 66, 11, 47;
                "紫薄汗", "zibohan" => 187, 161, 203;
                "退红", "tuihong" => 240, 207, 227;
                "昌荣", "changrong" => 220, 199, 225;
                "樱花", "yinhua" => 228, 184, 213;
                "丁香", "dingxiang" => 206, 147, 191;
                "木槿", "mujing" => 186, 121, 177;
                "茈藐", "zimiao" => 166, 126, 183;
                "紫蒲", "zipu" => 166, 85, 157;
                "紫紶", "ziqu" => 125, 82, 132;
                "拂紫绵", "fuzimian" => 126, 82, 127;
                "頳紫", "chengzi" => 138, 24, 116;
                "三公子", "sangongzi" => 102, 61, 116;
                "齐紫", "qizi" => 108, 33, 109;
                "凝夜紫", "ningyezi" => 66, 34, 86;
                "石英", "shiying" => 200, 182, 187;
                "香炉紫烟", "xiangluziyan" => 211, 204, 214;
                "苍烟落照", "cangyanluozhao" => 200, 181, 179;
                "甘石", "ganshi" => 189, 178, 178;
                "紫菂", "zidi" => 155, 142, 169;
                "银褐", "yinhe" => 156, 141, 155;
                "藕丝褐", "ousihe" => 168, 135, 135;
                "烟红", "yanhong" => 157, 133, 143;
                "迷楼灰", "milouhui" => 145, 130, 143;
                "红藤杖", "hongtengzhang" => 146, 129, 135;
                "鸦雏", "yachu" => 106, 91, 109;
                "玄天", "xuantian" => 107, 84, 88;
                "烟墨", "yanmo" => 92, 79, 85;
                "紫鼠", "zishu" => 89, 76, 87;
                // 黄色
                "栀子", "zhizi" => 250, 192, 61;
                "黄白游", "huangbaiyou" => 255, 247, 153;
                "松花", "songhua" => 255, 238, 111;
                "缃叶", "xiangye" => 236, 212, 82;
                "黄栗留", "huangliliu" => 254, 220, 94;
                "嫩鹅黄", "nengehuang" => 242, 200, 103;
                "黄河琉璃", "huangheliuli" => 229, 168, 75;
                "杏子", "xingzi" => 218, 146, 51;
                "红友", "hongyou" => 217, 136, 61;
                "库金", "kujing" => 225, 138, 59;
                "鞠衣", "juyi" => 211, 162, 55;
                "黄不老", "huangbulao" => 219, 155, 52;
                "郁金裙", "yujinqun" => 208, 134, 53;
                "露褐", "luhe" => 189, 130, 83;
                "鹅黄", "ehuang" => 190, 138, 47;
                "光明砂", "guangmingsha" => 204, 93, 32;
                "柘黄", "zhehuang" => 198, 121, 21;
                "媚蝶", "meidie" => 188, 110, 55;
                "黄流", "huangliu" => 159, 96, 39;
                "韎韐", "meige" => 159, 82, 33;
                "九斤黄", "jiujinghuang" => 221, 176, 120;
                "弗肯红", "fukenhong" => 236, 217, 199;
                "赤璋", "chizhang" => 225, 193, 153;
                "如梦令", "rumengling" => 221, 187, 153;
                "茧色", "jianse" => 198, 162, 104;
                "芸黄", "yunhuang" => 210, 163, 108;
                "椒房", "jiaofang" => 219, 156, 94;
                "金埒", "jinlie" => 190, 148, 87;
                "雌黄", "cihuang" => 180, 136, 77;
                "密陀僧", "mituoseng" => 179, 147, 75;
                "大块", "dakuai" => 191, 167, 130;
                "蜜合", "mihe" => 223, 215, 194;
                "地籁", "dilai" => 223, 206, 180;
                "仙米", "xianmi" => 212, 201, 170;
                "假山南", "jiashannan" => 212, 193, 166;
                "黄粱", "huangliang" => 196, 183, 152;
                "石蜜", "shimi" => 196, 191, 137;
                "紫花布", "zihuabu" => 190, 167, 139;
                "黄封", "huangfeng" => 202, 178, 114;
                "养生主", "yangshengzhu" => 180, 155, 127;
                "沙饧", "shaxing" => 191, 166, 112;
                "黄螺", "huangluo" => 180, 163, 121;
                "蒸栗", "zhengli" => 165, 138, 95;
                "巨吕", "julv" => 170, 142, 89;
                "降真香", "jiangzhenxiang" => 158, 131, 88;
                "大云", "dayun" => 148, 120, 79;
                "吉金", "jijin" => 137, 109, 71;
                "远志", "yuanzhi" => 129, 102, 59;
                "射干", "shegan" => 124, 98, 63;
                "油葫芦", "youhulu" => 100, 77, 49;
                "龙战", "longzhan" => 95, 67, 33;
                "緅絺", "zouchi" => 128, 76, 46;
                "葭灰", "jiahui" => 190, 177, 170;
                "珠子褐", "zhuzihe" => 190, 168, 157;
                "黄埃", "huangai" => 180, 146, 115;
                "沉香", "chenxiang" => 153, 128, 108;
                "明茶褐", "minchahe" => 158, 131, 104;
                "荆褐", "jinhe" => 144, 108, 74;
                "驼褐", "tuohe" => 124, 91, 62;
                "缊韨", "yunfu" => 152, 79, 49;
                "棠梨褐", "tanglihe" => 149, 90, 66;
                "檀褐", "tanhe" => 148, 86, 53;
                "朱石栗", "zhushihe" => 129, 73, 44;
                "紫瓯", "ziou" => 124, 70, 30;
                "栗壳", "like" => 119, 80, 57;
                "麝香褐", "shexianghe" => 105, 75, 60;
                "椒褐", "jiaohe" => 114, 69, 58;
                "枣褐", "zaohe" => 104, 54, 26;
                "目童子", "mutongzi" => 91, 50, 34;
                "青骊", "qingli" => 66, 37, 23;
                "丁香褐", "dingxianghe" => 189, 150, 131;
                "肉红", "rouhong" => 221, 197, 184;
                "夏籥", "xiayue" => 210, 175, 157;
                "檀唇", "tanchun" => 218, 158, 140;
                "紫磨金", "zimojin" => 186, 131, 107;
                "檀色", "tanse" => 178, 109, 93;
                "鹰背褐", "yinbeihe" => 143, 109, 95;
                "赭罗", "zheluo" => 154, 102, 85;
                "老僧衣", "laosengyi" => 164, 95, 68;
                "姜黄", "jianghuang" => 214, 197, 96;
                "半见", "banjian" => 255, 251, 199;
                "断肠", "duanchang" => 236, 235, 194;
                "葱青", "congqing" => 237, 241, 187;
                "女贞黄", "nvzhenhuang" => 247, 238, 173;
                "莺儿", "yinger" => 235, 225, 169;
                "桑蕾", "sanglei" => 234, 216, 154;
                "娟纨", "juanwan" => 236, 224, 147;
                "少艾", "shaoai" => 227, 235, 152;
                "绮钱", "qiqian" => 216, 222, 138;
                "翠樽", "cuizun" => 205, 209, 113;
                "田赤", "tianchi" => 225, 211, 132;
                "禹余粮", "yuyuliang" => 225, 210, 121;
                "姚黄", "yaohuang" => 214, 188, 70;
                "太一余粮", "taiyiyuliang" => 213, 180, 92;
                "栾华", "luanhua" => 192, 173, 94;
                "秋香", "qiuxiang" => 191, 156, 70;
                "大赤", "dachi" => 170, 150, 73;
                "苍黄", "canghuang" => 182, 160, 20;
                "老茯神", "laofusheng" => 170, 133, 52;
                "流黄", "liuhuang" => 139, 112, 66;
                "青白玉","qingbaiyu" => 202, 197, 160;
                "玉色", "yuse" => 234, 228, 209;
                "骨缥", "gupiao" => 235, 227, 199;
                "黄润", "huangrun" => 223, 214, 184;
                "缣缃", "jianxiang" => 213, 200, 160;
                "佩玖", "peijiu" => 172, 159, 138;
                "玄校", "xuanxiao" => 169, 160, 130;
                "黄琮", "huangcong" => 158, 140, 107;
                "石莲褐" ,"shilianhe"=> 146, 137, 123;
                "绿豆褐", "lvdouhe" => 146, 137, 107;
                "綟绶", "lishou" => 117, 108, 75;
                "茶色", "chase" => 136, 118, 87;
                "濯绛", "zhuojiang" => 121, 104, 96;
                "黑朱", "heizhu" => 112, 105, 93;
                "冥色", "mingse" => 102, 95, 77;
                "伽罗", "jialuo" => 109, 92, 61;
                "苍艾", "cangai" => 90, 76, 59
                    // 青色

            ],
        }
    }
}
