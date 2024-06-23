#[allow(unused)]
use anyhow::Result;
use serde_json::json;

#[tokio::test] 
async fn test_get_assets() -> Result<()>{
    let ht = httpc_test::new_client("http://localhost:3000")?;
    ht.do_get("/hello").await?.print().await?;
    Ok(())
}
#[tokio::test] 
async fn test_login() -> Result<()>{
    let ht = httpc_test::new_client("http://localhost:3000")?;
    ht.do_post("/login", json!({
        "username": "user",
        "password": "123"
    })).await?.print().await?;
    Ok(())
}