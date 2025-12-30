use log::{info, LevelFilter};
use journal::{Journal, JournalError};
use LevelFilter::Debug;

#[test]
fn info() -> Result<(), JournalError> {
    Journal::init(Debug)?;
    info!("info");
    Ok(())
}