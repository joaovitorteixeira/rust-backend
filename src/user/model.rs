use actix_web::web;
use diesel::{
    prelude::{Insertable, Queryable},
    query_dsl::methods::SelectDsl,
    RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{api_error::ApiError, db};

use super::schema::user;

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = crate::user::schema::user)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub email: String,
}

#[derive(Serialize, Deserialize, Insertable, ToSchema)]
#[diesel(table_name = crate::user::schema::user)]
pub struct UserCreate {
    pub email: String,
}

impl User {
    // TODO: find a better solution for the duplicated code
    pub async fn create(new_user: UserCreate) -> Result<(), ApiError> {
       let result = web::block(move || {
            let conn = &mut db::connection()?;
            diesel::insert_into(user::table)
                .values(&new_user)
                .execute(conn)?;
            Ok(())
        })
        .await;

        // SQLite does not return the created record
        match result {
            Err(e) => Err(ApiError::new(500, String::from(e.to_string()))),
            Ok(user) => user,
        }
    }

    pub async fn list() -> Result<Vec<Self>, ApiError> {
        let result = web::block(move || {
            let conn = &mut db::connection()?;
            let users = user::table.select(User::as_select()).load(conn)?;

            Ok(users)
        })
        .await;

        match result {
            Err(e) => Err(ApiError::new(500, String::from(e.to_string()))),
            Ok(users) => users,
        }
    }
}
