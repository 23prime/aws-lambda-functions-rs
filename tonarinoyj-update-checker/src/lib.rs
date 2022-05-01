use dotenv::dotenv;

type BoxError = Box<dyn std::error::Error>;

pub mod error;
pub mod event;
pub mod logger;

pub async fn run() -> Result<(), BoxError> {
    dotenv().ok();
    return Ok(());
}
