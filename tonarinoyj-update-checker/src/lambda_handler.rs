use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::info;

use tonarinoyj_update_checker::event::CloudWatchScheduledEvent;
use tonarinoyj_update_checker::{error, logger, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    logger::init();

    info!("######## START ########");
    lambda_runtime::run(service_fn(handler)).await?;
    info!("########  END  ########");

    return Ok(());
}

async fn handler(
    event: LambdaEvent<CloudWatchScheduledEvent>,
) -> Result<(), error::LambdaGeneralError> {
    info!("event: {:?}", event);

    let result = run().await;
    info!("result: {:?}", result);

    return match result {
        Ok(r) => Ok(r),
        Err(e) => Err(error::LambdaGeneralError::new(e.to_string())),
    };
}
