use tokio::net::TcpStream;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // let what_is_this = my_async_fn();
    // Nothing has been printed yet.
    // what_is_this.await;
    // Text has been printed and socket has been
    // established and closed.
    handle_map().await;
}

#[allow(dead_code)]
async fn my_async_fn() {
    println!("hello from async");
    let _socket = TcpStream::connect("127.0.0.1:3000").await.unwrap();
    println!("async TCP operation complete");
}

async fn handle_map() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("hello".to_string(), 42);

    // String 实现了 Borrow<str>，所以 &str 可以借用为 &str
    let key = "hello";  // &str
    if let Some(value) = map.get(key) {  // get 的签名: fn get<Q>(&self, k: &Q) where K: Borrow<Q>
        println!("Value: {}", value);  // 输出: 42，无需创建 String
    }
}