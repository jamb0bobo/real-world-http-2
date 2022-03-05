#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // https://github.com/http-rs/surf/issues/218#issuecomment-722028760
    let res = surf::head("https://httpbin.org/get").await?;
    dbg!(res.status());
    for name in res.header_names() {
        let s = format!("{}: {}",name, res.header(name).unwrap());
        dbg!(s);
    };
    Ok(())
}
