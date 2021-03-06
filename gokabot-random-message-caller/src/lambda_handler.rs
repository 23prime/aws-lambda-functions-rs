use std::env;

use futures::future::join_all;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::info;

use gokabot_random_message_caller::event::CloudWatchScheduledEvent;
use gokabot_random_message_caller::{error, gokabot, logger};

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

    let future_tasks = get_target_ids().into_iter().map(gokabot::call);
    let result = join_all(future_tasks).await;

    let has_error = result.into_iter().any(|r| r.is_err());

    if has_error {
        return Err(error::LambdaGeneralError::new(
            "Some errors occurred!".to_string(),
        ));
    } else {
        return Ok(());
    }
}

fn get_target_ids() -> Vec<String> {
    let target_ids_from_env = env::var("TARGET_IDS").expect("TARGET_IDS must be set");
    info!("target_ids: {:?}", target_ids_from_env);
    return target_ids_from_env
        .split(',')
        .map(|s| s.to_string())
        .collect();
}
