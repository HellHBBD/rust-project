use std::fs;
use toml_edit::{DocumentMut, Item};

fn main() {
    // 1. 讀取並解析
    let path = "Cargo.toml";
    let toml_str = fs::read_to_string(path).unwrap();
    let mut doc = toml_str.parse::<DocumentMut>().unwrap();

    // 2. 取出原本的 workspace.members（不帶 mut）
    let orig_arr = doc["workspace"]["members"].as_array().unwrap_or_else(|| {
        eprintln!("⚠️ 找不到 [workspace].members 陣列");
        std::process::exit(1);
    });

    // 3. 收集所有字串，並做不分大小寫排序
    let mut items: Vec<String> = orig_arr
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();
    items.sort_by_key(|s| s.to_lowercase()); // 案例不分大小寫排序

    // 4. 手動拼出多行陣列字串（去掉尾部多餘空白）
    let elems = items
        .iter()
        .map(|s| format!("\"{}\",", s))
        .collect::<Vec<_>>()
        .join("\n  ");
    let array_str = format!("[\n  {}\n]", elems); // <-- 注意這裡不要多一個空格

    // 5. 把它包成一個小 TOML，再取出 members 這個項目
    let snippet = format!("members = {}", array_str);
    let temp_doc = snippet
        .parse::<DocumentMut>() // 解析成 Document
        .expect("無法解析 members 陣列");
    let new_item: Item = temp_doc["members"].clone(); // clone 一份 Item

    // 6. 再寫回原 doc
    doc["workspace"]["members"] = new_item;

    // 6. 寫回檔案
    fs::write(path, doc.to_string()).unwrap();
    // println!("{}", doc.to_string());
    println!("✅ [workspace].members 已按字母排序並格式化為多行");
}
