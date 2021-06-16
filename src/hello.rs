use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_derive::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use log::{LevelFilter, error};
use simple_error::bail;


#[derive(Deserialize)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

#[allow(clippy::unnecessary_wraps)]
fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    lambda!(request_handler);
    Ok(())
}

fn request_handler(event: CustomEvent, context: Context) -> Result<Response, HandlerError> {
    if event.first_name.is_empty() {
        error!("Request {} missing first name", context.aws_request_id);
        bail!("Empty first name");
    }

    Ok(Response {
        message: format!("Hello, {}!", event.first_name),
    })
}