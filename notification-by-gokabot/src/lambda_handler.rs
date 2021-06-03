use lambda_runtime::{handler_fn, Context, Error};
use log::info;

use notification_by_gokabot::{error, event, logger, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    logger::init();

    info!("######## START ########");
    lambda_runtime::run(handler_fn(handler)).await?;
    info!("########  END  ########");

    return Ok(());
}

async fn handler(event: event::Event, _context: Context) -> Result<(), error::GokabotLambdaError> {
    info!("event => {:?}", event);

    let result = run(event).await;
    info!("result = {:?}", result);

    match result {
        Ok(r) => return Ok(r),
        Err(e) => return Err(error::GokabotLambdaError::new(e.to_string())),
    }
}
