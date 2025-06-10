use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

pub async fn try_until_works<T, E, Fut, F>(mut function: F) -> Result<T, E>
  where
      F: FnMut() -> Fut,
      Fut: Future<Output = Result<T, E>>,
  {
    let mut attempts = 0;
    loop {
      match function().await {
        Ok(val) => return Ok(val),
        Err(e) => {
          attempts += 1;
          if attempts >= 5 {
              return Err(e);
          }
          sleep(Duration::from_secs(1)).await;
          continue;
        }
      }
    }
}
