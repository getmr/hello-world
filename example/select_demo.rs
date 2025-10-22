use tokio::select;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let network_request = async {
        sleep(Duration::from_secs(2)).await;
        "网络响应".to_string()
    };

    let timeout = async {
        sleep(Duration::from_secs(1)).await;
        "超时".to_string()
    };

    select! {
        response = network_request => {
            println!("收到响应: {}", response);
        },
        _ = timeout => {
            println!("操作超时！");
        }
    }
    // 输出: 操作超时！（因为定时器先完成）
}