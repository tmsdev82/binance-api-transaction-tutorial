use hmac::{Hmac, Mac, NewMac};
use reqwest::{header, StatusCode};
use sha2::Sha256;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

fn get_client() -> reqwest::Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    headers.insert(
        header::HeaderName::from_static("x-mbx-apikey"),
        header::HeaderValue::from_str(&env::var("BINANCE_API_KEY").unwrap()).unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    client
}

fn get_timestamp(time: SystemTime) -> u128 {
    let since_epoch = time.duration_since(UNIX_EPOCH).unwrap();
    since_epoch.as_millis()
}

fn get_signature(request: String) -> String {
    let secret_key = env::var("BINANCE_SECRET_KEY").unwrap();
    let mut signed_key = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).unwrap();
    signed_key.update(request.as_bytes());
    let signature = hex::encode(signed_key.finalize().into_bytes());

    format!("{}", signature)
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    println!("key: {}", env::var("BINANCE_API_KEY").unwrap());

    // Get the timestamp and the signature
    let timestamp = get_timestamp(SystemTime::now());
    let params = format!("timestamp={}", timestamp.to_string());
    println!("Request: {}", &params);
    let signature = get_signature(params.clone());

    // Build the entire request URL including signature and timestamp
    let request = format!(
        "https://api.binance.com/api/v3/account?{}&signature={}",
        params.clone(),
        signature
    );

    // Send the request using the client
    let client = get_client();
    let result = client
        .get(request)
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    // Print the results
    println!("result: {:?}", result);
    println!("Positive coin balances: ");
    let balances = result["balances"].as_array().unwrap();
    for i in 0..balances.len() {
        let amount = balances[i]["free"]
            .as_str()
            .unwrap()
            .parse::<f32>()
            .unwrap();
        if amount > 0.0 {
            println!("{}: {}", balances[i]["asset"], amount);
        }
    }

    // Get the timestamp and the signature
    let timestamp = get_timestamp(SystemTime::now());
    let params = format!(
        "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.00300&recvWindow=5000&timestamp={}",
        timestamp.to_string()
    );
    println!("Request: {}", &params);
    let signature = get_signature(params.clone());

    // Build the entire request URL including signature and timestamp
    let request = format!(
        "https://api.binance.com/api/v3/order/test?{}&signature={}",
        params.clone(),
        signature.clone()
    );

    let result = client.post(request).send().await.unwrap();

    if result.status() == StatusCode::OK {
        println!("Status ok!");
        let data: serde_json::Value = result.json().await.unwrap();
        println!("Data: {}", data);
    } else {
        println!("An error occurred: {:?}", result);
        println!("Result text {}", result.text().await.unwrap());
        std::process::exit(1);
    }

    // Execute an actual order
    println!("Place an actual order and then cancel");
    // Get the timestamp and the signature
    let timestamp = get_timestamp(SystemTime::now());
    let params = format!(
        "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.00300&recvWindow=5000&timestamp={}",
        timestamp.to_string()
    );
    println!("Request: {}", &params);
    let signature = get_signature(params.clone());

    // Build the entire request URL including signature and timestamp
    let request = format!(
        "https://api.binance.com/api/v3/order?{}&signature={}",
        params.clone(),
        signature.clone()
    );

    let result = client.post(request).send().await.unwrap();

    if result.status() == StatusCode::OK {
        println!("Order status ok!");
        let data: serde_json::Value = result.json().await.unwrap();
        println!("Order data: {}", data);

        // Retrieve the order id, to use for cancelling the order
        let order_id = data["orderId"].to_string();

        // Cancel the order
        let timestamp = get_timestamp(SystemTime::now());
        let params = format!(
            "symbol=LTCBTC&orderId={}&recvWindow=5000&timestamp={}",
            order_id,
            timestamp.to_string()
        );
        println!("Request: {}", &params);
        let signature = get_signature(params.clone());

        // Build the entire request URL including signature and timestamp
        let request = format!(
            "https://api.binance.com/api/v3/order?{}&signature={}",
            params.clone(),
            signature.clone()
        );

        // Sending HTTP delete will cancel the order
        let result = client.delete(request).send().await.unwrap();
        if result.status() == StatusCode::OK {
            println!("Cancel order status ok!");
            let data: serde_json::Value = result.json().await.unwrap();
            println!("Cancel order data: {}", data);
        } else {
            println!("Cancel order: error occurred: {:?}", result);
            println!("Cancel order: result text {}", result.text().await.unwrap());
            std::process::exit(1);
        }
    } else {
        println!("Order: an error occurred: {:?}", result);
        println!("Order: result text {}", result.text().await.unwrap());
        std::process::exit(1);
    }
}
