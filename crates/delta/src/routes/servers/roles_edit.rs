use revolt_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, PartialRole, User,
};
use revolt_models::v0;
use revolt_permissions::{calculate_server_permissions, ChannelPermission};
use revolt_result::{create_error, Result};
use crate::util::json::{Json, Validate};
use rocket::State;

/// # Edit Role
///
/// Edit a role by its id.
#[openapi(tag = "Server Permissions")]
#[patch("/<target>/roles/<role_id>", data = "<data>", rank = 1)]
pub async fn edit(
    db: &State<Database>,
    user: User,
    target: Reference,
    role_id: String,
    data: Validate<Json<v0::DataEditRole>>,
) -> Result<Json<v0::Role>> {
    let data = data.into_inner().into_inner();

    let mut server = target.as_server(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    calculate_server_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::ManageRole)?;

    let member_rank = query.get_member_rank().unwrap_or(i64::MIN);

    if let Some(mut role) = server.roles.remove(&role_id) {
        // Prevent us from editing roles above us
        if role.rank <= member_rank {
            return Err(create_error!(NotElevated));
        }

        let v0::DataEditRole {
            name,
            colour,
            hoist,
            remove,
            ..
        } = data;

        let partial = PartialRole {
            name,
            colour,
            hoist,
            ..Default::default()
        };

        role.update(
            db,
            &server.id,
            &role_id,
            partial,
            remove
                .map(|v| v.into_iter().map(Into::into).collect())
                .unwrap_or_default(),
        )
        .await?;

        Ok(Json(role.into()))
    } else {
        Err(create_error!(NotFound))
    }
}
