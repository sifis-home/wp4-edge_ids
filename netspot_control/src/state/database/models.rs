use diesel::prelude::*;

#[derive(Queryable)]
pub struct Configuration {
    pub id: i32,
    pub config: String,
}
