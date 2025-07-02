use bson::{Document, to_bson};
use revolt_result::Result;
use iso8601_timestamp::Timestamp;

use crate::{MongoDb, AuthorizedBot, AuthorizedBotId};

use super::AbstractAuthorizedBots;

static COL: &str = "authorized_bots";

#[async_trait]
impl AbstractAuthorizedBots for MongoDb {
    /// Insert emoji into database.
    async fn insert_authorized_bot(&self, authorised_bot: &AuthorizedBot) -> Result<()> {
        query!(self, insert_one, COL, &authorised_bot).map(|_| ())
    }

    /// Fetch an emoji by its id
    async fn fetch_authorized_bot(&self, id: &AuthorizedBotId) -> Result<AuthorizedBot> {
        query!(self, find_one, COL, doc! { "id.user": &id.user, "id.bot": &id.bot })?.ok_or_else(|| create_error!(NotFound))
    }

    /// Fetch an emoji by its id
    async fn fetch_users_authorized_bots(&self, user_id: &str) -> Result<Vec<AuthorizedBot>> {
        query!(self, find, COL, doc! { "id.user": &user_id })
    }

    /// Deletes an authori
    async fn delete_authorized_bot(&self, id: &AuthorizedBotId) -> Result<()> {
        query!(self, delete_one, COL, doc! { "id.user": &id.user, "id.bot": &id.bot }).map(|_| ())
    }

    async fn deauthorize_authorized_bot(&self, id: &AuthorizedBotId) -> Result<AuthorizedBot> {
        self.col::<AuthorizedBot>(COL)
            .find_one_and_update(
            doc! {
                "id.user": &id.user,
                "id.bot": &id.bot
            },
            doc! {
                "$set": {
                    "unauthorized_at": to_bson(&Timestamp::now_utc()).unwrap()
                }
            }
            )
            .await
            .map_err(|_| create_database_error!("find_one_and_update", COL))
            .and_then(|opt| opt.ok_or_else(|| create_database_error!("find_one_and_update", COL)))
    }

    async fn fetch_deauthorized_authorized_bots(&self) -> Result<Vec<AuthorizedBot>> {
        query!(
            self,
            find,
            COL,
            doc! {
                "deauthorized_at": { "$exists": true }
            }
        )
    }
}