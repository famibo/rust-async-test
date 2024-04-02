use std::io::{Error, ErrorKind};

fn main() {

}

//Implementing general error handling (using std::io::Error )
async fn my_async_call(url: &str) -> Result<serde_json::Value, std::io::Error> {

    let response = reqwest::get(url)
        .await
        .map_err(|e| Error::new(ErrorKind::Other, format!("Could not retrieve response: {}", e)))?;

    let json = response
        .json::<serde_json::Value>()
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not decode to json"))?;
    //Ok(serde_json::Value::default())
    //Ok(serde_json::Value::from(0))
    Ok(json)
}

/*
async fn my_async_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    let res = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?;
    //Ok(serde_json::Value::default())
    Ok(res)
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async () {
        let url: &str = "https://cat-fact.herokuapp.com/facts/";
        let res = my_async_call(url).await;
        match res {
            Ok(r) => {
                dbg!("Async call returned: {}", r);
            },
            Err(e) => {
                dbg!("Async call failed: {}", e);
                //panic!("Async call failed");
            }
        };
    }
}
