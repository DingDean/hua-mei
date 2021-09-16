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
}

impl Colour {
    pub fn new(name: String, alias: String, r: u8, g: u8, b: u8) -> Self {
        Self {
            name,
            alias,
            rgb: vec![r, g, b],
            hex: format!("#{:02x}{:02x}{:02x}", r, g, b),
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
                "渌波", "lubo" => 155, 180, 150
            ],
        }
    }
}
