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
