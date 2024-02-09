use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::types::AttributeValue;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use tracing::info;
use chrono::Utc;
use std::convert::TryFrom;
use http::Request as HttpRequest;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub email: String,
    pub message: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// You can see more examples in Runtime's repository:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn handle_request(db_client: &Client, event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let body = event.body();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");
    //Log into Cloudwatch
    info!(payload = %s, "JSON Payload received");

    //Serialze JSON into struct.
    //If JSON is incorrect, send back 400 with error.
    let item = match serde_json::from_str::<Item>(s) {
        Ok(item) => item,
        Err(err) => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(err.to_string().into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    //Insert into the table.
    add_item(db_client, item.clone(), "zola_website_db").await?;

    //Deserialize into json to return in the Response
    let j = serde_json::to_string(&item)?;

    //Send back a 200 - success
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(j.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    //Get config from environment.
    let config = aws_config::load_from_env().await;
    //Create the DynamoDB client.
    let client = Client::new(&config);

    // run(service_fn(|event: Request| async {
    //     handle_request(&client, event).await
    // }))
    // .await

        // Hardcoded test event
    let request = HttpRequest::builder()
        .method("POST")
        .header("Content-Type", "application/json")
        .uri("/test/path")
        .body(Body::from("{\"name\": \"SpongeBob\", \"email\": \"pineapple@sea.com\", \"message\": \"Will this message deliver?\"}"))
        .expect("Failed to build request");

    // Convert http::Request to lambda_http::Request
    let lambda_request = Request::try_from(request).expect("Failed to convert to lambda_http::Request");

    // Use the hardcoded test event instead of processing incoming events
    let response = handle_request(&client, lambda_request).await?;
    println!("Response: {:?}", response);

    Ok(())
    
}

// Add an item to a table.
// snippet-start:[dynamodb.rust.add-item]
pub async fn add_item(client: &Client, item: Item, table: &str) -> Result<(), Error> {
    let email = item.email;
    let name_av = AttributeValue::S(item.name);
    let email_av = AttributeValue::S(email.clone());
    let message_av = AttributeValue::S(item.message);

    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S.%fZ").to_string();
    let email_timestamp = format!("{}#{}", email.clone(), timestamp);
    let email_timestamp_av = AttributeValue::S(email_timestamp);

    let request = client
        .put_item()
        .table_name(table)
        .item("email_timestamp", email_timestamp_av)
        .item("Name", name_av)
        .item("Email", email_av)
        .item("Content", message_av);

    info!("adding item to DynamoDB");

    let _resp = request.send().await?;

    Ok(())
}
