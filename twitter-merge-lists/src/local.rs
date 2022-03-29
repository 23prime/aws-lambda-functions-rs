use log::info;

use twitter_merge_lists::{logger, run};

type BoxError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    logger::init();

    info!("######## START ########");
    let result = run().await;
    info!("result: {:?}", result);
    info!("########  END  ########");

    return Ok(());
}
