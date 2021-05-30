use lambda_runtime::{handler_fn, Context, Error};
use log::info;

use notification_by_gokabot::{error, event, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler_fn(handler)).await?;
    return Ok(());
}

async fn handler(event: event::Event, _context: Context) -> Result<(), error::GokabotLambdaError> {
    info!("[handler] event => {:?}", event);

    let result = run(event).await;
    info!("[handler] result = {:?}", result);

    match result {
        Ok(r) => return Ok(r),
        Err(e) => return Err(error::from(e.to_string())),
    }
}
