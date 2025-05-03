use std::env;
use table_downloader::scrape_table_to_csv;
use url::Url;

#[tokio::main]
async fn main() {
    let webdriver_url = "http://localhost:43271";
    let output_csv = "output.csv";

    let cwd = env::current_dir().unwrap();
    let file_path = cwd.join("dynamic_table.html");
    let file_url = Url::from_file_path(&file_path).unwrap();
    scrape_table_to_csv(
        webdriver_url,
        file_url.as_str(),
        "//*[@id=\"dynamic-table\"]",
        output_csv,
    )
    .await;

    scrape_table_to_csv(
        webdriver_url,
        "https://www.basketball-reference.com/",
        "//*[@id=\"confs_standings_E\"]",
        output_csv,
    )
    .await;

    println!("✅ 表格已輸出到 {}", output_csv);
}
