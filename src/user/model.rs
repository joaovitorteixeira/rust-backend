use actix_web::web;
use diesel::{prelude::Queryable, query_dsl::methods::SelectDsl, RunQueryDsl, Selectable, SelectableHelper};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{api_error::ApiError, db};

use super::schema::user;

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = crate::user::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub email: String,
}

impl User {
    pub async  fn find_all() -> Result<Vec<Self>, ApiError> {
        let result = web::block(move || {
            let conn = &mut db::connection()?;
            let users = user::table.select(User::as_select()).load(conn)?;

            Ok(users)
        })
        .await;

        match result {
            Err(e) => Err(ApiError::new(500, String::from(e.to_string()))),
            Ok(users) => users
        }
    }
}