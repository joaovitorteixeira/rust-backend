use diesel::{prelude::Queryable, Selectable};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = crate::user::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub email: String,
}
