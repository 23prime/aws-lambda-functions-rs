use lambda_runtime::{handler_fn, Context, Error};
use log::info;

use twitter_followee_list::{error, event, logger, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    logger::init();

    info!("######## START ########");
    lambda_runtime::run(handler_fn(handler)).await?;
    info!("########  END  ########");

    return Ok(());
}

async fn handler(
    event: event::CloudWatchScheduledEvent,
    _context: Context,
) -> Result<(), error::LambdaGeneralError> {
    info!("event: {:?}", event);

    let result = run().await;
    info!("result: {:?}", result);

    match result {
        Ok(r) => return Ok(r),
        Err(e) => return Err(error::LambdaGeneralError::new(e.to_string())),
    }
}
