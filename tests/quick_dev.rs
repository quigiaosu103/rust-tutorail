use anyhow::Ok;
#[allow(unused)]
use anyhow::Result;
use serde_json::json;

#[tokio::test] 
async fn test_get_assets() -> Result<()>{
    let ht = httpc_test::new_client("http://localhost:3000")?;
    ht.do_get("/hello").await?.print().await?;
    Ok(())
}
async fn test_login() -> Result<()>{
    let ht = httpc_test::new_client("http://localhost:3000")?;
    ht.do_post("/login", json!({
        "username": "user",
        "password": "123"
    })).await?.print().await?;
    Ok(())
}

async fn test_create_ticket() -> Result<()> {
    let ht = httpc_test::new_client("http://localhost:3000")?;
    ht.do_post("/api/ticket", json!({
        "title": "ticket 1"
    })).await?.print().await?;
    ht.do_post("/api/ticket", json!({
        "title": "ticket 2"
    })).await?.print().await?;
    Ok(())
}

#[tokio::test]
async fn test_get_tickets() -> Result<()> {
    
    let ht = httpc_test::new_client("http://localhost:3000")?;
    ht.do_get("/api/tickets").await?.print().await?;
    Ok(())
}


#[tokio::test]
async fn test_delete_ticket() -> Result<()> {
    let ht = httpc_test::new_client("http://localhost:3000")?;
    ht.do_delete("/api/ticket/1").await?.print().await?;
    Ok(())
}

#[tokio::test]
async fn test_create() -> Result<()> {
    let ht = httpc_test::new_client("http://localhost:3000")?;
    ht.do_post("/login", json!({
        "username": "user",
        "password": "123"
    })).await?.print().await?;
    ht.do_post("/api/ticket", json!({
        "title": "ticket 1"
    })).await?.print().await?;
    ht.do_post("/api/ticket", json!({
        "title": "ticket 2"
    })).await?.print().await?;
    Ok(())
}