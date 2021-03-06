use lambda_runtime::{handler_fn, Context, Error};
use log::info;

use twitter_merge_lists::{error, event, logger, run};

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

    return match result {
        Ok(r) => Ok(r),
        Err(e) => Err(error::LambdaGeneralError::new(e.to_string())),
    };
}
