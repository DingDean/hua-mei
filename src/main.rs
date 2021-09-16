//! 中国传统色
//!
//! 我们首先需要做好颜色名称和色值的对应关系
//!
//! 首先，色值的原始值是 rgb 的，我们需要获得 hex，方便在 css 中使用
//! 其次，颜色名称是中文古风的，如何将他们和英文变量名对应起来也是一个问题
//!
//! 再有了对应关系后，我们希望将这层对应关系实际运用起来，比如:
//! * 打包成 tailwindcss 的一个插件
//! * 打包成 vim 的配色方案
//! * Rust 生态的色彩包
use handlebars::Handlebars;
use hua_mei::Tradition;

fn main() {
    let mut handlebar = Handlebars::new();
    let tradition = Tradition::new();
    handlebar
        .register_template_file("tailwind", "./templates/tailwind.hbs")
        .expect("unable to open template");

    let mut output = std::fs::File::create("./packages/tailwind/index.js")
        .expect("unable to create output file");

    handlebar
        .render_to_write("tailwind", &tradition, &mut output)
        .unwrap();
}
