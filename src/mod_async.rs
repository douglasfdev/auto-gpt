async fn my_async_call(url: &str) -> Result<serde_json::Value, std::io::Error> {
  let res: reqwest::Response = reqwest::get(url)
    .await
    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Could not retrieve response"))?;

  let json_response: serde_json::Value = res
  .json::<serde_json::Value>()
  .await
  .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Could not decode to JSON"))?;

  Ok(json_response)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn tests_calls_async_fn() {
    let api_url = "https://cat-fact.herokuapp.com/facts/";

    let my_res: Result<serde_json::Value, std::io::Error> = my_async_call(api_url).await;

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