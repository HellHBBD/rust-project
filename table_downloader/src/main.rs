use reqwest::header;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 模擬瀏覽器請求頭
    let headers = header::HeaderMap::new();
    // 通常需要添加 User-Agent 和 Referer
    // headers.insert(header::USER_AGENT, "Mozilla/5.0 ...".parse()?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // 替換為實際 API URL
    let api_url = "https://www.basketball-reference.com/";

    // 發送請求並解析 JSON
    let response = client.get(api_url).send().await?;
    let json_data: Value = response.json().await?;

    // 显式等待表格加载（关键步骤）
    let table_xpath = "//*[@id='confs_standings_E']";
    client
        .wait()
        .for_element(Locator::XPath(table_xpath))
        .await?;

    // 提取表格数据
    let table = client.find(Locator::XPath(table_xpath)).await?;

    // 获取所有行（跳过表头可根据需要调整）
    let rows = table.find_all(Locator::XPath("./tbody/tr")).await?;

    for row in rows {
        // 提取单元格文本
        let cells = row.find_all(Locator::XPath("./td")).await?;
        let mut row_data = Vec::new();

        for cell in cells {
            let text = cell.text().await?;
            row_data.push(text);
        }

        println!("{:?}", row_data);
    }

    // 关闭浏览器
    client.close().await?;
    Ok(())
}
