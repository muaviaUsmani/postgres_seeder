use diesel::prelude::*;
use std::time::SystemTime;
use serde_derive::{Serialize, Deserialize};

use crate::schema::{posts, users};

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub published_by_id: i32,
    pub metadata: serde_json::Value,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
