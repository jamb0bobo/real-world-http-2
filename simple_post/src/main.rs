use async_std::{fs::File, io::ReadExt};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct Ip {
    ip: String
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // struct
    // let data = Ip { ip: "127.0.0.1".into() };
    // dbg!(&data);
    // let mut res = surf::post("https://httpbin.org/post").body_json(&data)?.await?;

    // string
    // let data_str = "test data";
    // let mut res = surf::post("https://httpbin.org/post").body_string(data_str.into()).await?;
    // plain text
    // let data_str = "test data";
    // let mut res = surf::post("https://httpbin.org/post").header("Content-Type", "text/plain").body(data_str).await?;

    // form
    // let mut res = surf::post("https://httpbin.org/post").header("Content-Type", "application/x-www-form-urlencoded").body("body=foo").await?;

    // file upload
    let mut res = surf::post("https://httpbin.org/post")
        .body_file("tmp/kasutera.png").await?.await?;

    // file upload multilart/form-data but fail
    // let boundary = "----------------------------------------boundaryxyzboundaryxyz";
    // let mut data: Vec<u8> = Vec::new();
    // write!(data, "--{}\r\n", boundary)?;
    // write!(data, "Content-Disposition: form-data; name=\"smfile\"; filename=\"kasutera.png\"\r\n")?;
    // write!(data, "Content-Type: image/png\r\n")?;
    // write!(data, "\r\n")?;
    // let mut f = File::open("tmp/kasutera.png").await?;
    // f.read_to_end(&mut data);
    // write!(data, "\r\n")?;
    // write!(data, "--{}--\r\n", boundary)?;

    // let mut res = surf::post("https://httpbin.org/post")
    //     .header("Content-Type", format!("multipart/form-data; boundary={}", boundary))
    //     .body(data).await?;

    let body = res.body_string().await?;
    let v: Value = serde_json::from_str(&body)?;
    dbg!(v);
    Ok(())
}
