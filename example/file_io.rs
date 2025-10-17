use std::io::SeekFrom;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("foo.txt").await?;

    file.write_all("hello world! 你好！世界！".as_bytes()).await?;
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 40];

    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;
    let s = String::from_utf8_lossy(&buffer[..n]);
    println!("The string: {}", s);

    f.seek(SeekFrom::Start(0)).await?;
    let mut data_string = String::new();
    let m = f.read_to_string(&mut data_string).await?;
    println!("The string: {} 长度：{}", data_string, m);

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}