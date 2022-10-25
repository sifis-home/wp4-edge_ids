use super::schema::*;
use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct Configuration {
    pub id: i32,
    pub config: String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = configurations)]
pub struct NewConfiguration<'a> {
    pub config: &'a str,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = alarms)]
pub struct NewAlarms<'a> {
    pub time: i64,
    pub message: &'a str,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = data)]
pub struct NewData<'a> {
    pub time: i64,
    pub message: &'a str,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = webhooks)]
pub struct NewWebhook<'a> {
    pub config: &'a str,
}
