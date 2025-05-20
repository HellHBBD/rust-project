use table_downloader::{scrape_table, to_csv};

#[tokio::main]
async fn main() {
    let output_csv = "output.csv";
    let result = scrape_table(
        9515,
        "https://www.laerm-monitoring.de/zug/?mp=3/",
        "/html/body/div[1]/main/div/section/div/div/div[4]/table",
    )
    .await
    .unwrap();

    to_csv(&result, output_csv).unwrap();

    println!("✅ 表格已輸出到 {}", output_csv);
}
