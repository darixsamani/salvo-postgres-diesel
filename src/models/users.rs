use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use crate::models::schema::users;
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Debug, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub full_name: String,
}


// Insertable: represents new data to insert (no id)
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub full_name: String,
}