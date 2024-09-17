use hyper::{Client, Body, Request};

pub async fn cross_chain_communication(target_chain: &str, data: &str) {
    let client = Client::new();
    let req = Request::post(target_chain)
        .header("Content-Type", "application/json")
        .body(Body::from(data.to_string()))
        .unwrap();

    let _res = client.request(req).await;
}
