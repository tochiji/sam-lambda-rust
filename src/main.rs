use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};
use std::process;

#[tokio::main]
async fn main() {
    let func = service_fn(func);

    if let Err(err) = lambda_runtime::run(func).await {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let body = event["body"].as_str().unwrap_or("");
    let body_parsed: Value = serde_json::from_str(body).unwrap_or(Value::Null);
    let data = body_parsed["data"].as_str().unwrap_or("nothing");

    println!("event: {:?}", event);
    println!("body: {:?}", body);
    println!("body_parsed: {:?}", body_parsed);
    println!("data: {:?}", data);

    Ok(json!({ "result": format!("You Send {}!", data) }))
}
