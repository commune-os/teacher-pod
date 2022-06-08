use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct Content {
    pub id: i64,
    pub r#type: i32,
    pub title: String,
    pub source: String,
    pub author: i32,
    pub topic: i32,
    pub description: String,
    pub cover_image: String,
    pub up_date: sqlx::types::chrono::NaiveDate,
}

impl Content {
    pub const GET_CONTENT:&'static str = "select * from contents where id = $1;";
    pub const LATEST_12_CONTENT: &'static str = "select * from contents order by id desc limit 12;";
    pub const LATEST_12_CONTENT_BY_TOPIC: &'static str = "select * from contents where topic = $1 order by id desc limit 12;";
}