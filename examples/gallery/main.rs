mod page;

use dioxus::prelude::*;
use neo_diox::prelude::*;

use page::AppInner;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // 基础样式直接内联, 避免 FOUC
        document::Style { {RESET_STR} }
        document::Style { {STYLE_STR} }
        // 覆盖默认主题样式
        // document::Style{ {ThemeVars::default().to_css()} }
        // 用户样式自行导入, 以支持热重载
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        // 可导入额外的图标库
        document::Script { src: "https://code.iconify.design/3/3.1.0/iconify.min.js" }
        AppInner {}
    }
}
