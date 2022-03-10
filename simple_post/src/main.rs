use serde::{Deserialize, Serialize};
use serde_json::{Value};

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

    // form
    let mut res = surf::post("https://httpbin.org/post").header("Content-Type", "application/x-www-form-urlencoded").body("body=foo").await?;
    let body = res.body_string().await?;
    let v: Value = serde_json::from_str(&body)?;
    dbg!(v);
    Ok(())
}
