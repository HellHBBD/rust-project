use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::{self, time::sleep};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // 設定 Chrome WebDriver 參數
    let caps = DesiredCapabilities::chrome();

    // 啟動 WebDriver
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // 打開 Google 網站
    driver.goto("https://nckucc.webex.com/wbxmjs/joinservice/sites/nckucc/meeting/download/5b47c96e851b7e15dd241c2d9430d9e1").await?;
    driver
        .query(By::Id("broadcom-center-right"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::Escape, Click).unwrap();

    let join_button = driver.find(By::Id("broadcom-center-right")).await?;
    join_button.click().await?;

    driver
        .execute(
            r#"
        return new Promise(resolve => {
            if (document.readyState === 'complete') {
                resolve(true);
            } else {
                document.addEventListener('DOMContentLoaded', () => resolve(true));
            }
        });
        "#,
            vec![],
        )
        .await?;

    driver
        .query(By::XPath("//iframe[@id='ext-bar']"))
        .first()
        .await?
        .wait_until()
        .displayed()
        .await?;
    let iframe = driver.find(By::XPath("//iframe[@id='ext-bar']")).await?;
    iframe.enter_frame().await?;
    let elements = driver.find_all(By::XPath("//*")).await?;

    // 列出所有元素的 tag name
    for elem in elements {
        let tag = elem.class_name().await?;
        println!("Tag: {:?}", tag);
    }
    // let guest_button = driver
    //     .find(By::Css(
    //         "button[data-test='auth-crosslaunch-meeting-start-guest-join-button']",
    //     ))
    //     .await?;
    // guest_button.click().await?;

    // 關閉 WebDriver
    sleep(Duration::from_secs(3000000)).await;
    driver.quit().await?;
    Ok(())
}
