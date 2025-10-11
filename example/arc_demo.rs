use tokio::task::yield_now;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let rc = Arc::new("hello");

        // `rc` is used after `.await`. It must be persisted to
        // the task's state.
        yield_now().await;

        println!("{}", rc);
    });
}