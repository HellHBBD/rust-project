use csv::Writer;
use fantoccini::{ClientBuilder, Locator};

/// 抓取指定 URL 上，XPath 對應的表格內容並輸出為 CSV
pub async fn scrape_table_to_csv(
    webdriver_url: &str,
    page_url: &str,
    table_xpath: &str,
    output_csv: &str,
) {
    // 1. 建立 WebDriver 連線
    let client = ClientBuilder::native()
        .connect(webdriver_url)
        .await
        .expect("無法連線到 WebDriver");

    // 2. 打開指定頁面
    client.goto(page_url).await.unwrap();

    // 3. 等待表格節點出現
    client
        .wait()
        .for_element(Locator::XPath(table_xpath))
        .await
        .unwrap();

    let row_wait_xpath = format!("{}//tbody/tr", table_xpath);
    client
        .wait()
        .for_element(Locator::XPath(&row_wait_xpath))
        .await
        .unwrap();

    // 4. 找到所有 <tr> 行
    let row_xpath = format!("{}//tr", table_xpath);
    let rows = client.find_all(Locator::XPath(&row_xpath)).await.unwrap();

    // 5. 建立 CSV writer
    let mut wtr = Writer::from_path(output_csv).unwrap();

    // 6. 逐行抓取每個 <td> 或 <th> 的文字
    for row in rows {
        let cells = row.find_all(Locator::XPath(".//th|.//td")).await.unwrap();
        let mut record = Vec::with_capacity(cells.len());
        for cell in cells {
            let text = cell.text().await.unwrap();
            record.push(text);
        }
        wtr.write_record(&record).unwrap();
    }
    wtr.flush().unwrap();

    // 7. 關閉 session
    client.close().await.unwrap();
}
