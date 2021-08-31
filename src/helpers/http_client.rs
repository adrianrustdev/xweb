pub async fn http_client(url: String) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(res)
}