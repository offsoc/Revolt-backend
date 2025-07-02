use revolt_config::configure;
use revolt_database::DatabaseInfo;
use revolt_result::Result;
use tasks::{file_deletion, prune_dangling_files};
use tokio::try_join;

use crate::tasks::prune_authorized_bots;

pub mod tasks;

#[tokio::main]
async fn main() -> Result<()> {
    configure!(crond);

    let db = DatabaseInfo::Auto.connect().await.expect("database");
    try_join!(
        file_deletion::task(db.clone()),
        prune_dangling_files::task(db.clone()),
        prune_authorized_bots::task(db.clone()),
    )
    .map(|_| ())
}
