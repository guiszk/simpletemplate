use simpletemplate::render;

use serde_json::{json};
use std::fs;

fn main() {
    let data = json!({
        "name": ["Bob", "Belcher"], 
        "number": 12345,
        "color": "light purple",
        "show_items": "true",
        "show_foo": "false",
    });
    let content = fs::read_to_string("templates/index.html").expect("Error reading file.");
    let res = render(content.as_str(), data);
    println!("{}", res);
}