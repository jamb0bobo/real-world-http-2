use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Serialize, Deserialize)]
    struct Query {
        query: String
    }

    let query = Query { query: String::from("hello world") };

    let mut res = surf::get("https://httpbin.org/get").query(&query)?.await?;
    let body = res.body_string().await?;
    dbg!(&body);
    let v: Value = serde_json::from_str(&body)?;
    println!("{}", v);
    dbg!(v);

    // #[derive(Deserialize, Serialize)]
    // struct Ip {
    //     ip: String
    // }
    // let mut res = surf::get("https://api.ipify.org?format=json").await?;
    // let Ip { ip } = res.body_json().await?;
    // dbg!(ip);
    Ok(())
}
