use crate::{entity::*, handler::Error, user::User, DbConnExt};
use sea_orm::{prelude::*, ActiveModelTrait, ModelTrait};
use trillium::{Conn, Handler, Status};
use trillium_api::Json;
use trillium_caching_headers::CachingHeadersExt;
use trillium_router::RouterConnExt;

pub async fn index(conn: &mut Conn, account: Account) -> Result<impl Handler, Error> {
    let memberships = account.find_related(Memberships).all(conn.db()).await?;
    if let Some(last_modified) = memberships
        .iter()
        .map(|membership| membership.created_at)
        .max()
    {
        conn.set_last_modified(last_modified.into())
    }
    Ok(Json(memberships))
}

pub async fn create(
    conn: &mut Conn,
    (account, Json(membership)): (Account, Json<CreateMembership>),
) -> Result<impl Handler, Error> {
    let membership = membership.build(&account)?.insert(conn.db()).await?;
    Ok((Json(membership), Status::Created))
}

pub async fn delete(conn: &mut Conn, current_user: User) -> Result<impl Handler, Error> {
    let db = conn.db();

    let membership_id = conn.param("membership_id").unwrap();
    let membership_id = Uuid::parse_str(membership_id).map_err(|_| Error::NotFound)?;

    let (membership, account) = Memberships::find_by_id(membership_id)
        .find_also_related(Accounts)
        .one(db)
        .await?
        .ok_or(Error::NotFound)?;

    let account = account.ok_or(Error::NotFound)?;

    if membership.user_email == current_user.email {
        return Err(Error::AccessDenied);
    }

    if !current_user.is_admin() {
        account
            .find_related(Memberships)
            .filter(MembershipColumn::UserEmail.eq(&current_user.email))
            .one(db)
            .await?
            .ok_or(Error::NotFound)?;
    }

    membership.delete(db).await?;
    Ok(Status::NoContent)
}
