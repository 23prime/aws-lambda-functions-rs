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

    let mut has_error = false;

    let target_ids = vec!["MY_USER_ID", "NGA_GROUP_ID", "KMT_GROUP_ID"];

    for target_id in target_ids {
        let result = gokabot::call(target_id).await;
        info!("Call gokabot API [{}] => {:?}", target_id, result);

        if result.is_err() {
            has_error = true;
        }
    }

    if has_error {
        return Err(error::LambdaGeneralError::new(
            "Some errors occurred!".to_string(),
        ));
    } else {
        return Ok(());
    }
}
