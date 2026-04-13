use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/style/");

    let mut css = String::new();

    let root = PathBuf::from("src/style");

    let files = vec![
        "base.css",
        "container.css",
        "input.css",
        "selector.css",
        "navigation.css",
        "feedback.css",
        "data.css",
        "advanced.css",
    ];

    for file in files {
        let content = fs::read_to_string(root.join(file)).expect("Failed to read CSS file");
        css.push_str(&content);
        css.push_str("\n");
    }

    fs::write("src/style/style.css", &css).expect("Failed to write combined CSS file");
}
