async fn my_async_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {
  let res: serde_json::Value = reqwest::get(url)
    .await?
    .json::<serde_json::Value>()
    .await?;

  Ok(res)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn tests_calls_async_fn() {
    let api_url = "https://cat-fact.herokuapp.com/facts/";

    let my_res: Result<serde_json::Value, reqwest::Error> = my_async_call(api_url).await;
    match my_res {
        Ok(result) => {
          dbg!(result);
        },
        Err(e) => {
          panic!("Failed to make the request, {}", e)
        }
    }
  }
}