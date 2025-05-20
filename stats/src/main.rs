use dashmap::DashMap;
use std::sync::Arc;
use tokio::task;

use stats::get_weibull_days;
// use stats::get_ber_days;

const TASKS: usize = 10;
const INCREMENTS_PER_TASK: usize = 1000;

#[tokio::main]
async fn main() {
    let map = Arc::new(DashMap::new());
    let mut handles = vec![];

    for _ in 0..TASKS {
        let map = Arc::clone(&map);
        let handle = task::spawn_blocking(move || {
            for _ in 0..INCREMENTS_PER_TASK {
                // let key = get_ber_days();
                let key = get_weibull_days();
                map.entry(key).and_modify(|v| *v += 1).or_insert(1);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    // 複製為 Vec<(K, V)>
    let mut entries: Vec<_> = map
        .iter()
        .map(|entry| (entry.key().clone(), entry.value().clone()))
        .collect();

    // 根據 key 排序（也可以改成按 value 排）
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    // 列出所有 key 和累加次數
    for (key, value) in entries {
        println!("{key}: {value}");
    }
}
