use fantoccini::{ClientBuilder, Locator};

/// 抓取指定 URL 上，XPath 對應的表格內容並輸出為 CSV
pub async fn scrape_table(
    webdriver_url: &str,
    page_url: &str,
    table_xpath: &str,
) -> Result<Vec<Vec<String>>, ()> {
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

    let mut result = Vec::new();

    // 6. 逐行抓取每個 <td> 或 <th> 的文字
    for row in rows {
        let cells = row.find_all(Locator::XPath(".//th|.//td")).await.unwrap();
        let mut record = Vec::with_capacity(cells.len());
        for cell in cells {
            let text = cell.text().await.unwrap();
            record.push(text);
        }
        result.push(record);
    }

    // 7. 關閉 session
    client.close().await.unwrap();
    Ok(result)
}

#[cfg(test)]
mod scrape_table {
    use super::*;
    use std::env;
    use url::Url;

    #[tokio::test]
    async fn test_scrape_local_table() {
        let webdriver_url = "http://localhost:43271"; // 或你的 WebDriver 位置
        let cwd = env::current_dir().unwrap();
        let file_path = cwd.join("dynamic_table.html");
        let file_url = Url::from_file_path(&file_path).unwrap();

        let result = scrape_table(
            webdriver_url,
            file_url.as_str(),
            "//*[@id=\"dynamic-table\"]",
        )
        .await
        .unwrap();

        let expected = vec![
            vec!["ID", "名稱", "數值"],
            vec!["1", "Alice", "42"],
            vec!["2", "Bob", "73"],
            vec!["3", "Carol", "58"],
        ];

        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn test_scrape_basketball_reference() {
        let webdriver_url = "http://localhost:43271";
        let result = scrape_table(
            webdriver_url,
            "https://www.basketball-reference.com/",
            "//*[@id=\"confs_standings_E\"]",
        )
        .await
        .unwrap();

        let expected = vec![
            vec!["East", " ", " ", "W", "L"],
            vec!["CLE * (1) ", "F", "$", "64", "18"],
            vec!["BOS * (2) ", "F", "$", "61", "21"],
            vec!["NYK * (3) ", "F", "$", "51", "31"],
            vec!["IND * (4) ", "F", "$", "50", "32"],
            vec!["MIL * (5) ", "F", "$", "48", "34"],
            vec!["DET * (6) ", "F", "$", "44", "38"],
            vec!["ORL * (7) ", "F", "$", "41", "41"],
            vec!["ATL * (8) ", "F", "$", "40", "42"],
            vec!["CHI * (9) ", "F", "$", "39", "43"],
            vec!["MIA * (10) ", "F", "$", "37", "45"],
            vec!["TOR x (11) ", "F", "$", "30", "52"],
            vec!["BRK x (12) ", "F", "$", "26", "56"],
            vec!["PHI x (13) ", "F", "$", "24", "58"],
            vec!["CHO x (14) ", "F", "$", "19", "63"],
            vec!["WAS x (15) ", "F", "$", "18", "64"],
        ];

        assert_eq!(result, expected);
    }
}

pub fn to_csv(rows: &[Vec<String>], path: &str) -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_path(path)?;
    for row in rows {
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::to_csv;
    use std::fs;

    #[test]
    fn test_write_csv_output() {
        // 測試資料：必須使用 String
        let data: Vec<Vec<String>> = vec![
            vec!["Name".to_string(), "Score".to_string()],
            vec!["Alice".to_string(), "90".to_string()],
            vec!["Bob".to_string(), "80".to_string()],
        ];

        let expected = "Name,Score\nAlice,90\nBob,80\n";

        let path = "test_output.csv";
        // 呼叫並 unwrap 錯誤
        to_csv(&data, path).expect("Failed to write CSV");

        // 讀回並比對
        let actual = fs::read_to_string(path).expect("Failed to read CSV file");
        assert_eq!(
            actual, expected,
            "CSV content does not match expected output"
        );

        // 清理測試檔案
        let _ = fs::remove_file(path);
    }
}
